use atc_genesis_platform::{EntityId, Transform};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Parent(pub EntityId);

#[derive(Default)]
pub struct World {
    next: u64,
    transforms: HashMap<EntityId, Transform>,
    parents: HashMap<EntityId, Parent>,
}

impl World {
    pub fn new() -> Self { Self::default() }

    pub fn spawn(&mut self, transform: Transform) -> EntityId {
        self.next = self.next.saturating_add(1);
        let id = EntityId(self.next);
        self.transforms.insert(id, transform);
        id
    }

    pub fn despawn(&mut self, id: EntityId) -> bool {
        let removed = self.transforms.remove(&id).is_some();
        self.parents.remove(&id);
        self.parents.retain(|_, p| p.0 != id);
        removed
    }

    pub fn set_transform(&mut self, id: EntityId, transform: Transform) -> bool {
        if let Some(slot) = self.transforms.get_mut(&id) { *slot = transform; true } else { false }
    }

    pub fn transform(&self, id: EntityId) -> Option<&Transform> { self.transforms.get(&id) }
    pub fn set_parent(&mut self, child: EntityId, parent: Option<EntityId>) -> bool {
        if !self.transforms.contains_key(&child) || parent.is_some_and(|p| !self.transforms.contains_key(&p)) { return false; }
        match parent { Some(p) => { self.parents.insert(child, Parent(p)); }, None => { self.parents.remove(&child); } }
        true
    }
    pub fn parent(&self, child: EntityId) -> Option<EntityId> { self.parents.get(&child).map(|p| p.0) }
    pub fn len(&self) -> usize { self.transforms.len() }
    pub fn is_empty(&self) -> bool { self.transforms.is_empty() }

    pub fn iter_transforms(&self) -> impl Iterator<Item = (EntityId, &Transform)> { self.transforms.iter().map(|(id, t)| (*id, t)) }

    pub fn world_transform(&self, id: EntityId) -> Option<Transform> {
        let mut current = id;
        let mut result = *self.transforms.get(&id)?;
        let mut guard = 0;
        while let Some(parent) = self.parents.get(&current) {
            guard += 1;
            if guard > self.transforms.len() { return None; }
            let parent_transform = *self.transforms.get(&parent.0)?;
            result = combine(parent_transform, result);
            current = parent.0;
        }
        Some(result)
    }
}

fn combine(parent: Transform, local: Transform) -> Transform {
    Transform {
        translation: [
            parent.translation[0] + local.translation[0] * parent.scale[0],
            parent.translation[1] + local.translation[1] * parent.scale[1],
            parent.translation[2] + local.translation[2] * parent.scale[2],
        ],
        rotation_xyzw: quat_mul(parent.rotation_xyzw, local.rotation_xyzw),
        scale: [parent.scale[0] * local.scale[0], parent.scale[1] * local.scale[1], parent.scale[2] * local.scale[2]],
    }
}

fn quat_mul(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[3]*b[0]+a[0]*b[3]+a[1]*b[2]-a[2]*b[1], a[3]*b[1]-a[0]*b[2]+a[1]*b[3]+a[2]*b[0], a[3]*b[2]+a[0]*b[1]-a[1]*b[0]+a[2]*b[3], a[3]*b[3]-a[0]*b[0]-a[1]*b[1]-a[2]*b[2]]
}

pub struct TransformRenderPipeline<R> { pub world: World, pub renderer: R }

impl<R: atc_genesis_platform::Renderer> TransformRenderPipeline<R> {
    pub fn render(&mut self, frame: atc_genesis_platform::FrameId) {
        self.renderer.begin_frame(frame);
        for (id, _) in self.world.iter_transforms() {
            if let Some(world_transform) = self.world.world_transform(id) { self.renderer.submit(id, world_transform); }
        }
        self.renderer.end_frame();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hierarchy_resolves() {
        let mut w = World::new();
        let p = w.spawn(Transform { translation: [2.0,0.0,0.0], ..Default::default() });
        let c = w.spawn(Transform { translation: [1.0,0.0,0.0], ..Default::default() });
        assert!(w.set_parent(c, Some(p)));
        assert_eq!(w.world_transform(c).unwrap().translation[0], 3.0);
    }
}
