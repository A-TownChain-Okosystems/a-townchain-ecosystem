use atc_genesis_animation::{AnimationClip, AnimationClipId, AnimationPlayer, BoneId};
use atc_genesis_ecs::{World, WorldEcsBridge};
use atc_genesis_gameplay::{GameplayRuntime, MovementConfig};
use atc_genesis_input::{InputEvent, InputState};
use atc_genesis_network::{NetworkEntity, ReplicatedState, ReplicationTransport, Replicator, Tick};
use atc_genesis_physics::{PhysicsConfig, PhysicsSimulation, WorldPhysicsBridge};
use atc_genesis_platform::{AudioRuntime, EntityId, FrameId, Renderer, Transform};
use atc_genesis_world::{WorldChunk, WorldChunkId, WorldStreamer};
use std::collections::HashMap;

/// Deterministic coordinator for the core Genesis runtime pipeline.
pub struct GenesisRuntime<R, A = atc_genesis_audio::NullAudioRuntime, N = atc_genesis_network::LoopbackTransport> {
    pub world: WorldStreamer,
    pub ecs: World,
    pub physics: PhysicsSimulation,
    pub renderer: R,
    pub audio: A,
    pub input: InputState,
    pub gameplay: GameplayRuntime,
    pub network: Replicator<N>,
    network_tick: Tick,
    animations: HashMap<EntityId, AnimationBinding>,
    clips: HashMap<AnimationClipId, AnimationClip>,
    ecs_bridge: WorldEcsBridge,
    physics_bridge: WorldPhysicsBridge,
    frame: u64,
}

struct AnimationBinding {
    player: AnimationPlayer,
    clip: AnimationClipId,
    last_root_translation: [f32; 3],
}

impl<R> GenesisRuntime<R> {
    pub fn new(renderer: R, physics_config: PhysicsConfig) -> Self {
        Self::with_audio_and_network(renderer, atc_genesis_audio::NullAudioRuntime::default(), Replicator::new(atc_genesis_network::LoopbackTransport::default()), physics_config)
    }
}

impl<R, A, N> GenesisRuntime<R, A, N> {
    pub fn with_audio_and_network(renderer: R, audio: A, network: Replicator<N>, physics_config: PhysicsConfig) -> Self {
        Self { world: WorldStreamer::default(), ecs: World::new(), physics: PhysicsSimulation::new(physics_config), renderer, audio, input: InputState::default(), gameplay: GameplayRuntime::default(), network, network_tick: Tick(0), animations: HashMap::new(), clips: HashMap::new(), ecs_bridge: WorldEcsBridge::new(), physics_bridge: WorldPhysicsBridge::new(), frame: 0 }
    }
    pub fn frame(&self) -> u64 { self.frame }
    pub fn network_tick(&self) -> Tick { self.network_tick }
    pub fn add_chunk(&mut self, chunk: WorldChunk) -> Result<(), &'static str> { self.world.add_chunk(chunk) }
    pub fn spawn(&mut self, transform: Transform) -> EntityId { self.ecs.spawn(transform) }
    pub fn chunk_entity(&self, id: WorldChunkId) -> Option<EntityId> { self.ecs_bridge.entity_for_chunk(id) }
    pub fn stream(&mut self, position: [f32; 3], load_distance: f32, unload_distance: f32, budget: usize) -> Vec<WorldChunkId> { self.world.update(position, load_distance, unload_distance, budget) }
    pub fn input_event(&mut self, event: InputEvent) { self.input.apply(event); }
    pub fn queue_movement(&mut self, entity: EntityId, config: MovementConfig) { self.gameplay.sample_movement(&self.input, entity, config); }
    pub fn update_gameplay(&mut self, dt: f32) -> usize { self.gameplay.apply(&mut self.ecs, dt) }

    pub fn register_animation_clip(&mut self, clip: AnimationClip) -> Option<AnimationClip> { self.clips.insert(clip.id, clip) }

    pub fn play_animation(&mut self, entity: EntityId, clip: AnimationClipId, looping: bool, speed: f32) -> bool {
        if self.ecs.transform(entity).is_none() || !self.clips.contains_key(&clip) { return false; }
        self.animations.insert(entity, AnimationBinding { player: AnimationPlayer { clip: Some(clip), time_seconds: 0.0, speed, looping }, clip, last_root_translation: [0.0; 3] });
        true
    }

    pub fn stop_animation(&mut self, entity: EntityId) -> bool { self.animations.remove(&entity).is_some() }

    /// Advances animation clocks and applies root-motion deltas without accumulating
    /// the same sampled pose repeatedly across frames.
    pub fn update_animations(&mut self, dt: f32) -> usize {
        let ids: Vec<EntityId> = self.animations.keys().copied().collect();
        let mut updated = 0;
        for entity in ids {
            let Some(binding) = self.animations.get_mut(&entity) else { continue; };
            let Some(clip) = self.clips.get(&binding.clip) else { continue; };
            binding.player.update(dt, clip.duration_seconds);
            let root = binding.player.sample(clip).into_iter().find(|(bone, _)| *bone == BoneId(0));
            let Some((_, pose)) = root else { continue; };
            let delta = [
                pose.translation[0] - binding.last_root_translation[0],
                pose.translation[1] - binding.last_root_translation[1],
                pose.translation[2] - binding.last_root_translation[2],
            ];
            binding.last_root_translation = pose.translation;
            let Some(mut current) = self.ecs.transform(entity).copied() else { continue; };
            current.translation[0] += delta[0];
            current.translation[1] += delta[1];
            current.translation[2] += delta[2];
            current.rotation_xyzw = pose.rotation_xyzw;
            current.scale = pose.scale;
            if self.ecs.set_transform(entity, current) { updated += 1; }
        }
        updated
    }

    pub fn publish_entity(&mut self, entity: EntityId) -> Result<u64, String>
    where N: ReplicationTransport {
        let transform = self.ecs.transform(entity).ok_or_else(|| format!("entity {} does not exist", entity.0))?;
        let state = ReplicatedState { entity: NetworkEntity(entity.0), tick: self.network_tick, position_mm: quantize_mm(transform.translation), rotation_millirad: quantize_rotation(transform.rotation_xyzw) };
        self.network.publish(&state)
    }
}

impl<R: Renderer, A: AudioRuntime, N: ReplicationTransport> GenesisRuntime<R, A, N> {
    pub fn tick(&mut self, dt: f32) -> u32 {
        self.update_animations(dt);
        self.update_gameplay(dt);
        self.ecs_bridge.sync(&self.world, &mut self.ecs);
        self.physics_bridge.sync(&self.world, &mut self.physics);
        let steps = self.physics.advance(dt);
        self.physics.sync_to_world(&mut self.ecs);
        self.audio.update(dt.max(0.0));
        let frame = FrameId(self.frame);
        self.renderer.begin_frame(frame);
        for (entity, transform) in self.ecs.query_world_transforms() { self.renderer.submit(entity, transform); }
        self.renderer.end_frame();
        self.network_tick = Tick(self.network_tick.0.saturating_add(1));
        self.frame = self.frame.saturating_add(1);
        steps
    }
}

fn quantize_mm(values: [f32; 3]) -> [i32; 3] { values.map(|v| (v.clamp(i32::MIN as f32 / 1000.0, i32::MAX as f32 / 1000.0) * 1000.0).round() as i32) }
fn quantize_rotation(rotation_xyzw: [f32; 4]) -> [i32; 3] { [rotation_xyzw[0], rotation_xyzw[1], rotation_xyzw[2]].map(|v| (v.clamp(-2.147, 2.147) * 1_000_000.0).round() as i32) }

#[cfg(test)]
mod tests {
    use super::*;
    use atc_genesis_animation::{AnimationTrack, Keyframe, PoseTransform};
    use atc_genesis_audio::NullAudioRuntime;
    use atc_genesis_input::{InputEvent, Key};
    use atc_genesis_platform::{AssetId, EntityId};
    use atc_genesis_renderer::NullRenderer;
    use atc_genesis_world::{ChunkState, WorldBounds};

    fn loaded_chunk(id: u64) -> WorldChunk { WorldChunk { id: WorldChunkId(id), asset: AssetId(id as u128), bounds: WorldBounds { min: [0.0; 3], max: [10.0; 3] }, state: ChunkState::Loaded } }

    #[test]
    fn tick_mirrors_loaded_chunk_and_advances_network_tick() {
        let mut runtime = GenesisRuntime::new(NullRenderer::default(), PhysicsConfig { gravity: [0.0; 3], ..Default::default() });
        runtime.add_chunk(loaded_chunk(7)).unwrap();
        assert_eq!(runtime.tick(1.0 / 60.0), 1);
        assert_eq!(runtime.chunk_entity(WorldChunkId(7)), Some(EntityId(u64::MAX - 7)));
        assert_eq!(runtime.physics.query(atc_genesis_physics::Aabb { min: [1.0; 3], max: [2.0; 3] }), vec![EntityId(u64::MAX - 7)]);
        assert_eq!(runtime.renderer.submitted_count(), 1);
        assert_eq!(runtime.frame(), 1);
        assert_eq!(runtime.network_tick(), Tick(1));
    }

    #[test]
    fn gameplay_moves_entity_before_render() {
        let mut runtime = GenesisRuntime::new(NullRenderer::default(), PhysicsConfig { gravity: [0.0; 3], ..Default::default() });
        let entity = runtime.spawn(Transform::default());
        runtime.input_event(InputEvent::KeyPressed(Key::Right));
        runtime.queue_movement(entity, MovementConfig { speed: 2.0, ..Default::default() });
        assert_eq!(runtime.tick(0.5), 0);
        assert_eq!(runtime.ecs.transform(entity).unwrap().translation[0], 1.0);
    }

    #[test]
    fn animation_root_motion_does_not_accumulate_sampled_pose() {
        let mut runtime = GenesisRuntime::new(NullRenderer::default(), PhysicsConfig { gravity: [0.0; 3], ..Default::default() });
        let entity = runtime.spawn(Transform::default());
        runtime.register_animation_clip(AnimationClip { id: AnimationClipId(1), duration_seconds: 1.0, tracks: vec![AnimationTrack { bone: BoneId(0), keys: vec![Keyframe { time_seconds: 0.0, value: PoseTransform::default() }, Keyframe { time_seconds: 1.0, value: PoseTransform { translation: [2.0, 0.0, 0.0], ..Default::default() } }] }] });
        assert!(runtime.play_animation(entity, AnimationClipId(1), false, 1.0));
        runtime.update_animations(0.5);
        assert_eq!(runtime.ecs.transform(entity).unwrap().translation[0], 1.0);
        runtime.update_animations(0.5);
        assert_eq!(runtime.ecs.transform(entity).unwrap().translation[0], 2.0);
    }

    #[test]
    fn entity_snapshot_is_published() {
        let mut runtime = GenesisRuntime::new(NullRenderer::default(), PhysicsConfig { gravity: [0.0; 3], ..Default::default() });
        let entity = runtime.spawn(Transform { translation: [1.23456, -2.0004, 3.5], ..Default::default() });
        assert_eq!(runtime.publish_entity(entity).unwrap(), 1);
        assert_eq!(runtime.network.transport.drain().len(), 1);
    }

    #[test]
    fn custom_audio_is_updated_each_tick() {
        #[derive(Default)] struct CountingAudio { updates: u32 }
        impl AudioRuntime for CountingAudio { fn update(&mut self, _: f32) { self.updates += 1; } fn set_master_gain(&mut self, _: f32) {} }
        let mut runtime = GenesisRuntime::with_audio_and_network(NullRenderer::default(), CountingAudio::default(), Replicator::new(atc_genesis_network::LoopbackTransport::default()), PhysicsConfig::default());
        runtime.tick(0.016);
        assert_eq!(runtime.audio.updates, 1);
    }

    #[test]
    fn explicit_audio_runtime_can_be_constructed() {
        let runtime = GenesisRuntime::with_audio_and_network(NullRenderer::default(), NullAudioRuntime::default(), Replicator::new(atc_genesis_network::LoopbackTransport::default()), PhysicsConfig::default());
        assert_eq!(runtime.frame(), 0);
    }
}
