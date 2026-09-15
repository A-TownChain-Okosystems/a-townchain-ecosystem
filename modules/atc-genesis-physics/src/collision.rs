use atc_genesis_platform::EntityId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb { pub min: [f32; 3], pub max: [f32; 3] }
impl Aabb {
    pub fn contains(&self, p: [f32; 3]) -> bool { (0..3).all(|i| p[i] >= self.min[i] && p[i] <= self.max[i]) }
    pub fn intersects(&self, other: &Self) -> bool { (0..3).all(|i| self.min[i] <= other.max[i] && self.max[i] >= other.min[i]) }
    pub fn ray_intersection(&self, origin: [f32; 3], direction: [f32; 3], max_distance: f32) -> Option<f32> {
        if max_distance < 0.0 { return None; }
        let mut near = 0.0;
        let mut far = max_distance;
        for i in 0..3 {
            if direction[i].abs() < f32::EPSILON {
                if origin[i] < self.min[i] || origin[i] > self.max[i] { return None; }
                continue;
            }
            let inv = 1.0 / direction[i];
            let mut t1 = (self.min[i] - origin[i]) * inv;
            let mut t2 = (self.max[i] - origin[i]) * inv;
            if t1 > t2 { std::mem::swap(&mut t1, &mut t2); }
            near = near.max(t1);
            far = far.min(t2);
            if near > far { return None; }
        }
        Some(near)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Collider { pub entity: EntityId, pub bounds: Aabb }

#[derive(Default)]
pub struct CollisionWorld { colliders: Vec<Collider> }
impl CollisionWorld {
    pub fn add(&mut self, collider: Collider) { self.colliders.push(collider); }
    pub fn query_aabb(&self, bounds: Aabb) -> Vec<EntityId> { self.colliders.iter().filter(|c| c.bounds.intersects(&bounds)).map(|c| c.entity).collect() }
    pub fn raycast(&self, origin: [f32; 3], direction: [f32; 3], max_distance: f32) -> Option<EntityId> {
        self.colliders.iter().filter_map(|c| c.bounds.ray_intersection(origin, direction, max_distance).map(|t| (t, c.entity))).min_by(|a,b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)).map(|(_, id)| id)
    }
    pub fn clear(&mut self) { self.colliders.clear(); }
}
