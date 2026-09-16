use atc_genesis_ecs::{World, WorldEcsBridge};
use atc_genesis_physics::{PhysicsConfig, PhysicsSimulation, WorldPhysicsBridge};
use atc_genesis_platform::{FrameId, Renderer, Transform};
use atc_genesis_world::{WorldChunk, WorldChunkId, WorldStreamer};

/// Deterministic coordinator for the core Genesis runtime pipeline.
///
/// Tick order is fixed:
/// 1. world streaming
/// 2. world -> ECS synchronization
/// 3. world -> physics synchronization
/// 4. fixed-step physics
/// 5. physics -> ECS synchronization
/// 6. renderer submission
pub struct GenesisRuntime<R> {
    pub world: WorldStreamer,
    pub ecs: World,
    pub physics: PhysicsSimulation,
    pub renderer: R,
    ecs_bridge: WorldEcsBridge,
    physics_bridge: WorldPhysicsBridge,
    frame: u64,
}

impl<R> GenesisRuntime<R> {
    pub fn new(renderer: R, physics_config: PhysicsConfig) -> Self {
        Self {
            world: WorldStreamer::default(),
            ecs: World::new(),
            physics: PhysicsSimulation::new(physics_config),
            renderer,
            ecs_bridge: WorldEcsBridge::new(),
            physics_bridge: WorldPhysicsBridge::new(),
            frame: 0,
        }
    }

    pub fn frame(&self) -> u64 {
        self.frame
    }

    pub fn add_chunk(&mut self, chunk: WorldChunk) -> Result<(), &'static str> {
        self.world.add_chunk(chunk)
    }

    pub fn spawn(&mut self, transform: Transform) -> atc_genesis_platform::EntityId {
        self.ecs.spawn(transform)
    }

    pub fn chunk_entity(&self, id: WorldChunkId) -> Option<atc_genesis_platform::EntityId> {
        self.ecs_bridge.entity_for_chunk(id)
    }

    pub fn stream(&mut self, position: [f32; 3], load_distance: f32, unload_distance: f32, budget: usize) -> Vec<WorldChunkId> {
        self.world.update(position, load_distance, unload_distance, budget)
    }
}

impl<R: Renderer> GenesisRuntime<R> {
    /// Execute one deterministic runtime tick.
    pub fn tick(&mut self, dt: f32) -> u32 {
        self.ecs_bridge.sync(&self.world, &mut self.ecs);
        self.physics_bridge.sync(&self.world, &mut self.physics);

        let steps = self.physics.advance(dt);
        self.physics.sync_to_world(&mut self.ecs);

        let frame = FrameId(self.frame);
        self.renderer.begin_frame(frame);
        for (entity, transform) in self.ecs.query_world_transforms() {
            self.renderer.submit(entity, transform);
        }
        self.renderer.end_frame();
        self.frame = self.frame.saturating_add(1);
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atc_genesis_platform::{AssetId, EntityId};
    use atc_genesis_renderer::NullRenderer;
    use atc_genesis_world::{ChunkState, WorldBounds};

    fn loaded_chunk(id: u64) -> WorldChunk {
        WorldChunk {
            id: WorldChunkId(id),
            asset: AssetId(id as u128),
            bounds: WorldBounds { min: [0.0; 3], max: [10.0; 3] },
            state: ChunkState::Loaded,
        }
    }

    #[test]
    fn tick_mirrors_loaded_chunk_into_ecs_physics_and_renderer() {
        let mut runtime = GenesisRuntime::new(NullRenderer::default(), PhysicsConfig { gravity: [0.0; 3], ..Default::default() });
        runtime.add_chunk(loaded_chunk(7)).unwrap();
        assert_eq!(runtime.tick(1.0 / 60.0), 1);
        assert_eq!(runtime.chunk_entity(WorldChunkId(7)), Some(EntityId(u64::MAX - 7)));
        assert_eq!(runtime.physics.query(atc_genesis_physics::Aabb { min: [1.0; 3], max: [2.0; 3] }), vec![EntityId(u64::MAX - 7)]);
        assert_eq!(runtime.renderer.submitted_count(), 1);
        assert_eq!(runtime.frame(), 1);
    }

    #[test]
    fn tick_order_is_stable_for_multiple_entities() {
        let mut runtime = GenesisRuntime::new(NullRenderer::default(), PhysicsConfig { gravity: [0.0; 3], ..Default::default() });
        runtime.spawn(Transform { translation: [3.0, 0.0, 0.0], ..Default::default() });
        runtime.spawn(Transform { translation: [1.0, 0.0, 0.0], ..Default::default() });
        assert_eq!(runtime.tick(0.0), 0);
        assert_eq!(runtime.renderer.graph.commands()[0].entity, EntityId(1));
        assert_eq!(runtime.renderer.graph.commands()[1].entity, EntityId(2));
    }
}
