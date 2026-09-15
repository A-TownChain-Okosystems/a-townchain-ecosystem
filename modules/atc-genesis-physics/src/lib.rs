use atc_genesis_platform::{EntityId, PhysicsWorld};

#[derive(Default)]
pub struct NullPhysicsWorld {
    steps: u64,
}

impl NullPhysicsWorld {
    pub fn steps(&self) -> u64 { self.steps }
}

impl PhysicsWorld for NullPhysicsWorld {
    fn step(&mut self, _dt_seconds: f32) { self.steps = self.steps.saturating_add(1); }

    fn raycast(&self, _origin: [f32; 3], _direction: [f32; 3], _max_distance: f32) -> Option<EntityId> {
        None
    }
}
