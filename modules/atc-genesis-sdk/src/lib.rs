use atc_genesis_platform::{EntityId, Transform};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeStep {
    pub fixed_seconds: f32,
    pub frame: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpawnedEntity(pub EntityId);

pub trait GamePlugin {
    fn name(&self) -> &'static str;
    fn startup(&mut self, _ctx: &mut GameContext) {}
    fn update(&mut self, _ctx: &mut GameContext, _time: TimeStep) {}
    fn shutdown(&mut self, _ctx: &mut GameContext) {}
}

#[derive(Default)]
pub struct GameContext {
    next_entity: u64,
    transforms: Vec<(EntityId, Transform)>,
}

impl GameContext {
    pub fn new() -> Self { Self { next_entity: 1, transforms: Vec::new() } }
    pub fn spawn(&mut self, transform: Transform) -> SpawnedEntity {
        let id = EntityId(self.next_entity);
        self.next_entity = self.next_entity.saturating_add(1);
        self.transforms.push((id, transform));
        SpawnedEntity(id)
    }
    pub fn transform(&self, id: EntityId) -> Option<Transform> {
        self.transforms.iter().find(|(entity, _)| *entity == id).map(|(_, t)| *t)
    }
    pub fn entity_count(&self) -> usize { self.transforms.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn entities_get_stable_ids() {
        let mut ctx = GameContext::new();
        assert_eq!(ctx.spawn(Transform::default()).0, EntityId(1));
        assert_eq!(ctx.spawn(Transform::default()).0, EntityId(2));
    }
}
