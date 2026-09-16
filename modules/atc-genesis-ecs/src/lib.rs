use atc_genesis_platform::{EntityId, FrameId, Renderer, Transform};
use atc_genesis_world::{ChunkState, WorldChunkId, WorldStreamer};
use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Parent(pub EntityId);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityRecord { pub id: EntityId }

trait ComponentTable: Any + Send + Sync {
    fn remove_entity(&mut self, id: EntityId);
    fn contains_entity(&self, id: EntityId) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct TypedComponentTable<T: Any + Send + Sync> { values: HashMap<EntityId, T> }
impl<T: Any + Send + Sync> ComponentTable for TypedComponentTable<T> {
    fn remove_entity(&mut self, id: EntityId) { self.values.remove(&id); }
    fn contains_entity(&self, id: EntityId) -> bool { self.values.contains_key(&id) }
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

#[derive(Default)]
pub struct World {
    next: u64,
    transforms: HashMap<EntityId, Transform>,
    parents: HashMap<EntityId, Parent>,
    components: HashMap<TypeId, Box<dyn ComponentTable>>,
}

impl World {
    pub fn new() -> Self { Self::default() }

    pub fn spawn(&mut self, transform: Transform) -> EntityId {
        self.next = self.next.saturating_add(1);
        let id = EntityId(self.next);
        self.transforms.insert(id, transform);
        id
    }

    pub fn insert(&mut self, id: EntityId, transform: Transform) -> bool {
        if self.transforms.contains_key(&id) { return false; }
        self.next = self.next.max(id.0);
        self.transforms.insert(id, transform);
        true
    }

    pub fn contains(&self, id: EntityId) -> bool { self.transforms.contains_key(&id) }

    pub fn despawn(&mut self, id: EntityId) -> bool {
        let removed = self.transforms.remove(&id).is_some();
        self.parents.remove(&id);
        self.parents.retain(|_, p| p.0 != id);
        for table in self.components.values_mut() { table.remove_entity(id); }
        removed
    }

    pub fn set_transform(&mut self, id: EntityId, transform: Transform) -> bool {
        if let Some(slot) = self.transforms.get_mut(&id) { *slot = transform; true } else { false }
    }
    pub fn transform(&self, id: EntityId) -> Option<&Transform> { self.transforms.get(&id) }

    pub fn insert_component<T: Any + Send + Sync>(&mut self, id: EntityId, component: T) -> bool {
        if !self.contains(id) { return false; }
        let table = self.components.entry(TypeId::of::<T>()).or_insert_with(|| Box::new(TypedComponentTable::<T> { values: HashMap::new() }));
        table.as_any_mut().downcast_mut::<TypedComponentTable<T>>().expect("component type table invariant").values.insert(id, component).is_none()
    }

    pub fn set_component<T: Any + Send + Sync>(&mut self, id: EntityId, component: T) -> bool {
        if !self.contains(id) { return false; }
        let table = self.components.entry(TypeId::of::<T>()).or_insert_with(|| Box::new(TypedComponentTable::<T> { values: HashMap::new() }));
        table.as_any_mut().downcast_mut::<TypedComponentTable<T>>().expect("component type table invariant").values.insert(id, component).is_some()
    }

    pub fn component<T: Any + Send + Sync>(&self, id: EntityId) -> Option<&T> {
        self.components.get(&TypeId::of::<T>())?.as_any().downcast_ref::<TypedComponentTable<T>>()?.values.get(&id)
    }

    pub fn component_mut<T: Any + Send + Sync>(&mut self, id: EntityId) -> Option<&mut T> {
        self.components.get_mut(&TypeId::of::<T>())?.as_any_mut().downcast_mut::<TypedComponentTable<T>>()?.values.get_mut(&id)
    }

    pub fn remove_component<T: Any + Send + Sync>(&mut self, id: EntityId) -> Option<T> {
        self.components.get_mut(&TypeId::of::<T>())?.as_any_mut().downcast_mut::<TypedComponentTable<T>>()?.values.remove(&id)
    }

    pub fn has_component<T: Any + Send + Sync>(&self, id: EntityId) -> bool {
        self.components.get(&TypeId::of::<T>()).is_some_and(|table| table.contains_entity(id))
    }

    pub fn set_parent(&mut self, child: EntityId, parent: Option<EntityId>) -> bool {
        if !self.contains(child) || parent == Some(child) || parent.is_some_and(|p| !self.contains(p)) { return false; }
        if let Some(p) = parent { if self.would_cycle(child, p) { return false; } self.parents.insert(child, Parent(p)); } else { self.parents.remove(&child); }
        true
    }
    fn would_cycle(&self, child: EntityId, proposed_parent: EntityId) -> bool {
        let mut current = proposed_parent;
        for _ in 0..=self.parents.len() { if current == child { return true; } match self.parents.get(&current) { Some(p) => current = p.0, None => return false } }
        true
    }
    pub fn parent(&self, child: EntityId) -> Option<EntityId> { self.parents.get(&child).map(|p| p.0) }
    pub fn len(&self) -> usize { self.transforms.len() }
    pub fn is_empty(&self) -> bool { self.transforms.is_empty() }
    pub fn entities(&self) -> Vec<EntityId> { let mut ids: Vec<_> = self.transforms.keys().copied().collect(); ids.sort_by_key(|id| id.0); ids }
    pub fn iter_transforms(&self) -> impl Iterator<Item = (EntityId, &Transform)> {
        let mut ids: Vec<_> = self.transforms.keys().copied().collect(); ids.sort_by_key(|id| id.0);
        ids.into_iter().filter_map(|id| self.transforms.get(&id).map(|t| (id, t)))
    }
    pub fn world_transform(&self, id: EntityId) -> Option<Transform> {
        if !self.contains(id) { return None; }
        let mut chain = Vec::new(); let mut current = id;
        for _ in 0..=self.parents.len() { chain.push(current); match self.parents.get(&current) { Some(p) => current = p.0, None => break } }
        if chain.len() > self.parents.len() + 1 { return None; }
        let mut result = *self.transforms.get(chain.last()?)?;
        for entity in chain.iter().rev().skip(1) { result = combine(result, *self.transforms.get(entity)?); }
        Some(result)
    }
    pub fn query_world_transforms(&self) -> Vec<(EntityId, Transform)> { self.entities().into_iter().filter_map(|id| self.world_transform(id).map(|t| (id, t))).collect() }
}

fn combine(parent: Transform, local: Transform) -> Transform { Transform { translation: [parent.translation[0] + local.translation[0] * parent.scale[0], parent.translation[1] + local.translation[1] * parent.scale[1], parent.translation[2] + local.translation[2] * parent.scale[2]], rotation_xyzw: quat_mul(parent.rotation_xyzw, local.rotation_xyzw), scale: [parent.scale[0] * local.scale[0], parent.scale[1] * local.scale[1], parent.scale[2] * local.scale[2]] } }
fn quat_mul(a: [f32; 4], b: [f32; 4]) -> [f32; 4] { [a[3]*b[0]+a[0]*b[3]+a[1]*b[2]-a[2]*b[1], a[3]*b[1]-a[0]*b[2]+a[1]*b[3]+a[2]*b[0], a[3]*b[2]+a[0]*b[1]-a[1]*b[0]+a[2]*b[3], a[3]*b[3]-a[0]*b[0]-a[1]*b[1]-a[2]*b[2]] }

pub struct TransformRenderPipeline<R> { pub world: World, pub renderer: R }
impl<R: Renderer> TransformRenderPipeline<R> { pub fn render(&mut self, frame: FrameId) { self.renderer.begin_frame(frame); for (id, transform) in self.world.query_world_transforms() { self.renderer.submit(id, transform); } self.renderer.end_frame(); } }

pub struct WorldEcsBridge { chunk_entities: HashMap<WorldChunkId, EntityId> }
impl Default for WorldEcsBridge { fn default() -> Self { Self { chunk_entities: HashMap::new() } } }
impl WorldEcsBridge {
    pub fn new() -> Self { Self::default() }
    pub fn entity_for_chunk(&self, chunk: WorldChunkId) -> Option<EntityId> { self.chunk_entities.get(&chunk).copied() }
    pub fn sync(&mut self, world: &WorldStreamer, ecs: &mut World) -> Vec<(WorldChunkId, EntityId)> {
        let mut loaded = Vec::new();
        for chunk in world.chunks() {
            if chunk.state != ChunkState::Loaded { continue; }
            let entity = *self.chunk_entities.entry(chunk.id).or_insert_with(|| EntityId(u64::MAX - chunk.id.0));
            let center = [(chunk.bounds.min[0] + chunk.bounds.max[0]) * 0.5, (chunk.bounds.min[1] + chunk.bounds.max[1]) * 0.5, (chunk.bounds.min[2] + chunk.bounds.max[2]) * 0.5];
            let transform = Transform { translation: center, ..Default::default() };
            if !ecs.set_transform(entity, transform) && !ecs.insert(entity, transform) { continue; }
            loaded.push((chunk.id, entity));
        }
        let active: std::collections::HashSet<_> = world.chunks().iter().filter(|c| c.state == ChunkState::Loaded).map(|c| c.id).collect();
        let stale: Vec<_> = self.chunk_entities.keys().copied().filter(|id| !active.contains(id)).collect();
        for chunk in stale { if let Some(entity) = self.chunk_entities.remove(&chunk) { ecs.despawn(entity); } }
        loaded.sort_by_key(|(chunk, _)| chunk.0); loaded
    }
}

#[cfg(test)]
mod tests {
    use super::*; use atc_genesis_platform::AssetId; use atc_genesis_world::{WorldBounds, WorldChunk};
    #[derive(Debug, PartialEq)] struct Health(u32);
    #[derive(Debug, PartialEq)] struct Name(&'static str);
    #[test] fn hierarchy_resolves() { let mut w=World::new(); let p=w.spawn(Transform{translation:[2.0,0.0,0.0],..Default::default()}); let c=w.spawn(Transform{translation:[1.0,0.0,0.0],..Default::default()}); assert!(w.set_parent(c,Some(p))); assert_eq!(w.world_transform(c).unwrap().translation[0],3.0); }
    #[test] fn cycle_is_rejected() { let mut w=World::new(); let a=w.spawn(Transform::default()); let b=w.spawn(Transform::default()); assert!(w.set_parent(b,Some(a))); assert!(!w.set_parent(a,Some(b))); }
    #[test] fn missing_entity_has_no_world_transform() { let w=World::new(); assert!(w.world_transform(EntityId(999)).is_none()); }
    #[test] fn imported_id_is_preserved() { let mut w=World::new(); assert!(w.insert(EntityId(42),Transform::default())); assert_eq!(w.transform(EntityId(42)),Some(&Transform::default())); }
    #[test] fn loaded_chunk_is_mirrored_into_ecs() { let mut streamer=WorldStreamer::default(); streamer.add_chunk(WorldChunk{id:WorldChunkId(7),asset:AssetId(7),bounds:WorldBounds{min:[0.0;3],max:[10.0;3]},state:ChunkState::Loaded}).unwrap(); let mut ecs=World::new(); let mut bridge=WorldEcsBridge::new(); let result=bridge.sync(&streamer,&mut ecs); assert_eq!(result.len(),1); let entity=result[0].1; assert_eq!(ecs.transform(entity).unwrap().translation,[5.0;3]); assert_eq!(bridge.entity_for_chunk(WorldChunkId(7)),Some(entity)); }
    #[test] fn loaded_chunk_entity_is_stable() { let mut streamer=WorldStreamer::default(); streamer.add_chunk(WorldChunk{id:WorldChunkId(7),asset:AssetId(7),bounds:WorldBounds{min:[0.0;3],max:[10.0;3]},state:ChunkState::Loaded}).unwrap(); let mut ecs=World::new(); let mut bridge=WorldEcsBridge::new(); let first=bridge.sync(&streamer,&mut ecs)[0].1; let second=bridge.sync(&streamer,&mut ecs)[0].1; assert_eq!(first,second); }
    #[test] fn chunk_entity_does_not_collide_with_normal_spawn_range() { let mut ecs=World::new(); let normal=ecs.spawn(Transform::default()); assert_ne!(normal,EntityId(u64::MAX-7)); }
    #[test] fn generic_components_are_inserted_and_read() { let mut w=World::new(); let e=w.spawn(Transform::default()); assert!(w.insert_component(e, Health(100))); assert_eq!(w.component::<Health>(e),Some(&Health(100))); assert!(w.has_component::<Health>(e)); }
    #[test] fn generic_components_are_mutable_and_replaceable() { let mut w=World::new(); let e=w.spawn(Transform::default()); assert!(w.insert_component(e, Health(100))); assert!(w.set_component(e, Health(80))); w.component_mut::<Health>(e).unwrap().0 -= 5; assert_eq!(w.component::<Health>(e),Some(&Health(75))); }
    #[test] fn generic_components_are_removed_with_entity() { let mut w=World::new(); let e=w.spawn(Transform::default()); assert!(w.insert_component(e, Name("player"))); assert_eq!(w.remove_component::<Name>(e),Some(Name("player"))); assert!(!w.has_component::<Name>(e)); assert!(!w.remove_component::<Name>(e).is_some()); }
    #[test] fn despawn_cleans_generic_components() { let mut w=World::new(); let e=w.spawn(Transform::default()); assert!(w.insert_component(e, Health(1))); assert!(w.despawn(e)); assert!(!w.contains(e)); assert!(!w.has_component::<Health>(e)); }
    #[test] fn transform_iteration_is_deterministic() { let mut w=World::new(); let a=w.spawn(Transform::default()); let b=w.spawn(Transform::default()); let ids: Vec<_>=w.iter_transforms().map(|(id,_)| id).collect(); assert_eq!(ids,vec![a,b]); }
}