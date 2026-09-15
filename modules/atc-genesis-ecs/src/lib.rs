use atc_genesis_platform::{EntityId, FrameId, Renderer, Transform};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Parent(pub EntityId);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityRecord { pub id: EntityId }

#[derive(Default)]
pub struct World {
    next: u64,
    transforms: HashMap<EntityId, Transform>,
    parents: HashMap<EntityId, Parent>,
}

impl World {
    pub fn new() -> Self { Self::default() }
    pub fn spawn(&mut self, transform: Transform) -> EntityId { self.next = self.next.saturating_add(1); let id = EntityId(self.next); self.transforms.insert(id, transform); id }
    pub fn insert(&mut self, id: EntityId, transform: Transform) -> bool { if self.transforms.contains_key(&id) { return false; } self.next = self.next.max(id.0); self.transforms.insert(id, transform); true }
    pub fn despawn(&mut self, id: EntityId) -> bool { let removed = self.transforms.remove(&id).is_some(); self.parents.remove(&id); self.parents.retain(|_, p| p.0 != id); removed }
    pub fn set_transform(&mut self, id: EntityId, transform: Transform) -> bool { if let Some(slot) = self.transforms.get_mut(&id) { *slot = transform; true } else { false } }
    pub fn transform(&self, id: EntityId) -> Option<&Transform> { self.transforms.get(&id) }
    pub fn set_parent(&mut self, child: EntityId, parent: Option<EntityId>) -> bool {
        if !self.transforms.contains_key(&child) || parent == Some(child) || parent.is_some_and(|p| !self.transforms.contains_key(&p)) { return false; }
        if let Some(p) = parent { if self.would_cycle(child, p) { return false; } self.parents.insert(child, Parent(p)); } else { self.parents.remove(&child); }
        true
    }
    fn would_cycle(&self, child: EntityId, proposed_parent: EntityId) -> bool {
        let mut current = proposed_parent;
        for _ in 0..=self.transforms.len() { if current == child { return true; } match self.parents.get(&current) { Some(p) => current = p.0, None => return false } }
        true
    }
    pub fn parent(&self, child: EntityId) -> Option<EntityId> { self.parents.get(&child).map(|p| p.0) }
    pub fn len(&self) -> usize { self.transforms.len() }
    pub fn is_empty(&self) -> bool { self.transforms.is_empty() }
    pub fn entities(&self) -> Vec<EntityId> { let mut ids: Vec<_> = self.transforms.keys().copied().collect(); ids.sort_by_key(|id| id.0); ids }
    pub fn iter_transforms(&self) -> impl Iterator<Item = (EntityId, &Transform)> { self.transforms.iter().map(|(id, t)| (*id, t)) }
    pub fn world_transform(&self, id: EntityId) -> Option<Transform> {
        let mut chain = Vec::new(); let mut current = id;
        for _ in 0..=self.transforms.len() { chain.push(current); match self.parents.get(&current) { Some(p) => current = p.0, None => break } }
        if chain.len() > self.transforms.len() + 1 { return None; }
        let mut result = *self.transforms.get(chain.last()?)?;
        for entity in chain.iter().rev().skip(1) { result = combine(result, *self.transforms.get(entity)?); }
        Some(result)
    }
    pub fn query_world_transforms(&self) -> Vec<(EntityId, Transform)> { self.entities().into_iter().filter_map(|id| self.world_transform(id).map(|t| (id, t))).collect() }
}

fn combine(parent: Transform, local: Transform) -> Transform {
    Transform { translation: [parent.translation[0] + local.translation[0] * parent.scale[0], parent.translation[1] + local.translation[1] * parent.scale[1], parent.translation[2] + local.translation[2] * parent.scale[2]], rotation_xyzw: quat_mul(parent.rotation_xyzw, local.rotation_xyzw), scale: [parent.scale[0] * local.scale[0], parent.scale[1] * local.scale[1], parent.scale[2] * local.scale[2]] }
}
fn quat_mul(a: [f32; 4], b: [f32; 4]) -> [f32; 4] { [a[3]*b[0]+a[0]*b[3]+a[1]*b[2]-a[2]*b[1], a[3]*b[1]-a[0]*b[2]+a[1]*b[3]+a[2]*b[0], a[3]*b[2]+a[0]*b[1]-a[1]*b[0]+a[2]*b[3], a[3]*b[3]-a[0]*b[0]-a[1]*b[1]-a[2]*b[2]] }

pub struct TransformRenderPipeline<R> { pub world: World, pub renderer: R }
impl<R: Renderer> TransformRenderPipeline<R> {
    pub fn render(&mut self, frame: FrameId) { self.renderer.begin_frame(frame); for (id, transform) in self.world.query_world_transforms() { self.renderer.submit(id, transform); } self.renderer.end_frame(); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn hierarchy_resolves() { let mut w=World::new(); let p=w.spawn(Transform{translation:[2.0,0.0,0.0],..Default::default()}); let c=w.spawn(Transform{translation:[1.0,0.0,0.0],..Default::default()}); assert!(w.set_parent(c,Some(p))); assert_eq!(w.world_transform(c).unwrap().translation[0],3.0); }
    #[test] fn cycle_is_rejected() { let mut w=World::new(); let a=w.spawn(Transform::default()); let b=w.spawn(Transform::default()); assert!(w.set_parent(b,Some(a))); assert!(!w.set_parent(a,Some(b))); }
    #[test] fn imported_id_is_preserved() { let mut w=World::new(); assert!(w.insert(EntityId(42),Transform::default())); assert_eq!(w.transform(EntityId(42)),Some(&Transform::default())); }
}
