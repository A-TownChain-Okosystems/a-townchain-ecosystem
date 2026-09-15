use atc_genesis_platform::EntityId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb { pub min: [f32;3], pub max: [f32;3] }

impl Aabb {
    pub fn contains(&self, p: [f32;3]) -> bool { (0..3).all(|i| p[i] >= self.min[i] && p[i] <= self.max[i]) }
    pub fn intersects(&self, other: &Self) -> bool { (0..3).all(|i| self.min[i] <= other.max[i] && self.max[i] >= other.min[i]) }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Collider { pub entity: EntityId, pub bounds: Aabb }

#[derive(Default)]
pub struct CollisionWorld { colliders: Vec<Collider> }
impl CollisionWorld {
    pub fn add(&mut self, collider: Collider) { self.colliders.push(collider); }
    pub fn query_aabb(&self, bounds: Aabb) -> Vec<EntityId> { self.colliders.iter().filter(|c| c.bounds.intersects(&bounds)).map(|c| c.entity).collect() }
    pub fn clear(&mut self) { self.colliders.clear(); }
}
