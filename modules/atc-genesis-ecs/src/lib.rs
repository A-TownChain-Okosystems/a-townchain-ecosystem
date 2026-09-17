use atc_genesis_platform::{EntityId, FrameId, Renderer, Transform};
use atc_genesis_world::{ChunkState, WorldChunkId, WorldStreamer};
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Parent(pub EntityId);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityRecord { pub id: EntityId }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleEvent {
    Spawned(EntityId),
    Despawned(EntityId),
    ComponentInserted { entity: EntityId, component: TypeId },
    ComponentRemoved { entity: EntityId, component: TypeId },
}

pub trait LifecycleHook: Send + Sync {
    fn on_event(&mut self, event: LifecycleEvent);
}

#[derive(Default)]
pub struct LifecycleHooks {
    hooks: Vec<Box<dyn LifecycleHook>>,
    events: Vec<LifecycleEvent>,
}

impl LifecycleHooks {
    pub fn register<H: LifecycleHook + 'static>(&mut self, hook: H) { self.hooks.push(Box::new(hook)); }
    fn emit(&mut self, event: LifecycleEvent) { for hook in &mut self.hooks { hook.on_event(event); } self.events.push(event); }
    pub fn events(&self) -> &[LifecycleEvent] { &self.events }
    pub fn drain_events(&mut self) -> Vec<LifecycleEvent> { std::mem::take(&mut self.events) }
}

trait ComponentTable: Any + Send + Sync {
    fn remove_entity(&mut self, id: EntityId);
    fn contains_entity(&self, id: EntityId) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn clear_changes(&mut self);
    fn is_added(&self, id: EntityId) -> bool;
    fn is_changed(&self, id: EntityId) -> bool;
}

struct TypedComponentTable<T: Any + Send + Sync> {
    values: HashMap<EntityId, T>,
    added: HashSet<EntityId>,
    changed: HashSet<EntityId>,
}

impl<T: Any + Send + Sync> ComponentTable for TypedComponentTable<T> {
    fn remove_entity(&mut self, id: EntityId) { self.values.remove(&id); self.added.remove(&id); self.changed.remove(&id); }
    fn contains_entity(&self, id: EntityId) -> bool { self.values.contains_key(&id) }
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn clear_changes(&mut self) { self.added.clear(); self.changed.clear(); }
    fn is_added(&self, id: EntityId) -> bool { self.added.contains(&id) }
    fn is_changed(&self, id: EntityId) -> bool { self.changed.contains(&id) }
}

#[derive(Default)]
pub struct ResourceStore { values: HashMap<TypeId, Box<dyn Any + Send + Sync>> }
impl ResourceStore {
    pub fn insert<T: Any + Send + Sync>(&mut self, value: T) -> Option<T> { self.values.insert(TypeId::of::<T>(), Box::new(value)).and_then(|v| v.downcast::<T>().ok().map(|v| *v)) }
    pub fn get<T: Any + Send + Sync>(&self) -> Option<&T> { self.values.get(&TypeId::of::<T>()).and_then(|v| v.downcast_ref()) }
    pub fn get_mut<T: Any + Send + Sync>(&mut self) -> Option<&mut T> { self.values.get_mut(&TypeId::of::<T>()).and_then(|v| v.downcast_mut()) }
    pub fn remove<T: Any + Send + Sync>(&mut self) -> Option<T> { self.values.remove(&TypeId::of::<T>()).and_then(|v| v.downcast::<T>().ok().map(|v| *v)) }
    pub fn contains<T: Any + Send + Sync>(&self) -> bool { self.values.contains_key(&TypeId::of::<T>()) }
    pub fn len(&self) -> usize { self.values.len() }
    pub fn is_empty(&self) -> bool { self.values.is_empty() }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SystemId(pub u32);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SystemDescriptor { pub id: SystemId, pub name: String, before: Vec<SystemId>, after: Vec<SystemId> }
impl SystemDescriptor {
    pub fn new(id: SystemId, name: impl Into<String>) -> Self { Self { id, name: name.into(), before: Vec::new(), after: Vec::new() } }
    pub fn before(mut self, system: SystemId) -> Self { self.before.push(system); self }
    pub fn after(mut self, system: SystemId) -> Self { self.after.push(system); self }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScheduleError { DuplicateSystem(SystemId), DependencyCycle(Vec<SystemId>), MissingDependency { system: SystemId, dependency: SystemId } }

#[derive(Default)]
pub struct SystemSchedule { systems: HashMap<SystemId, SystemDescriptor> }
impl SystemSchedule {
    pub fn new() -> Self { Self::default() }
    pub fn register(&mut self, d: SystemDescriptor) -> Result<(), ScheduleError> { if self.systems.contains_key(&d.id) { return Err(ScheduleError::DuplicateSystem(d.id)); } self.systems.insert(d.id, d); Ok(()) }
    pub fn len(&self) -> usize { self.systems.len() }
    pub fn is_empty(&self) -> bool { self.systems.is_empty() }
    pub fn get(&self, id: SystemId) -> Option<&SystemDescriptor> { self.systems.get(&id) }
    pub fn ordered_systems(&self) -> Result<Vec<SystemId>, ScheduleError> {
        let mut incoming: HashMap<_, HashSet<_>> = self.systems.keys().copied().map(|id| (id, HashSet::new())).collect();
        for d in self.systems.values() {
            for dep in &d.after { if !self.systems.contains_key(dep) { return Err(ScheduleError::MissingDependency { system: d.id, dependency: *dep }); } incoming.get_mut(&d.id).unwrap().insert(*dep); }
            for dep in &d.before { if !self.systems.contains_key(dep) { return Err(ScheduleError::MissingDependency { system: d.id, dependency: *dep }); } incoming.get_mut(dep).unwrap().insert(d.id); }
        }
        let mut ready: Vec<_> = incoming.iter().filter_map(|(id, deps)| deps.is_empty().then_some(*id)).collect(); ready.sort();
        let mut ordered = Vec::with_capacity(self.systems.len());
        while let Some(id) = ready.first().copied() {
            ready.remove(0); ordered.push(id);
            let dependents: Vec<_> = incoming.iter().filter_map(|(candidate, deps)| deps.contains(&id).then_some(*candidate)).collect();
            for dependent in dependents { let deps = incoming.get_mut(&dependent).unwrap(); deps.remove(&id); if deps.is_empty() { ready.push(dependent); } }
            ready.sort();
        }
        if ordered.len() != self.systems.len() { let mut cycle: Vec<_> = incoming.iter().filter_map(|(id, deps)| (!deps.is_empty()).then_some(*id)).collect(); cycle.sort(); return Err(ScheduleError::DependencyCycle(cycle)); }
        Ok(ordered)
    }
}

pub type SystemFn = Box<dyn FnMut(&mut World) + Send + 'static>;
pub struct ExecutableSystem { pub descriptor: SystemDescriptor, pub run: SystemFn }

#[derive(Default)]
pub struct SystemExecutor { systems: HashMap<SystemId, ExecutableSystem> }
impl SystemExecutor {
    pub fn new() -> Self { Self::default() }
    pub fn register<F>(&mut self, descriptor: SystemDescriptor, run: F) -> Result<(), ScheduleError> where F: FnMut(&mut World) + Send + 'static { if self.systems.contains_key(&descriptor.id) { return Err(ScheduleError::DuplicateSystem(descriptor.id)); } self.systems.insert(descriptor.id, ExecutableSystem { descriptor, run: Box::new(run) }); Ok(()) }
    pub fn len(&self) -> usize { self.systems.len() }
    pub fn run(&mut self, world: &mut World) -> Result<Vec<SystemId>, ScheduleError> {
        let mut schedule = SystemSchedule::new(); for system in self.systems.values() { schedule.register(system.descriptor.clone())?; }
        let order = schedule.ordered_systems()?; for id in &order { self.systems.get_mut(id).expect("scheduled system invariant").run.as_mut()(world); } Ok(order)
    }
}

#[derive(Default)]
pub struct World {
    next: u64,
    transforms: HashMap<EntityId, Transform>,
    parents: HashMap<EntityId, Parent>,
    components: HashMap<TypeId, Box<dyn ComponentTable>>,
    resources: ResourceStore,
    lifecycle: LifecycleHooks,
}

impl World {
    pub fn new() -> Self { Self::default() }
    pub fn spawn(&mut self, transform: Transform) -> EntityId { self.next = self.next.saturating_add(1); let id = EntityId(self.next); self.transforms.insert(id, transform); self.lifecycle.emit(LifecycleEvent::Spawned(id)); id }
    pub fn insert(&mut self, id: EntityId, transform: Transform) -> bool { if self.transforms.contains_key(&id) { return false; } self.next = self.next.max(id.0); self.transforms.insert(id, transform); self.lifecycle.emit(LifecycleEvent::Spawned(id)); true }
    pub fn contains(&self, id: EntityId) -> bool { self.transforms.contains_key(&id) }
    pub fn despawn(&mut self, id: EntityId) -> bool { if self.transforms.remove(&id).is_none() { return false; } self.parents.remove(&id); self.parents.retain(|_, p| p.0 != id); for table in self.components.values_mut() { table.remove_entity(id); } self.lifecycle.emit(LifecycleEvent::Despawned(id)); true }
    pub fn set_transform(&mut self, id: EntityId, transform: Transform) -> bool { if let Some(slot) = self.transforms.get_mut(&id) { *slot = transform; true } else { false } }
    pub fn transform(&self, id: EntityId) -> Option<&Transform> { self.transforms.get(&id) }
    fn component_table<T: Any + Send + Sync>(&mut self) -> &mut TypedComponentTable<T> { self.components.entry(TypeId::of::<T>()).or_insert_with(|| Box::new(TypedComponentTable { values: HashMap::new(), added: HashSet::new(), changed: HashSet::new() })).as_any_mut().downcast_mut::<TypedComponentTable<T>>().expect("component type table invariant") }
    pub fn insert_component<T: Any + Send + Sync>(&mut self, id: EntityId, component: T) -> bool { if !self.contains(id) { return false; } let type_id=TypeId::of::<T>(); let table=self.component_table::<T>(); let inserted=table.values.insert(id,component).is_none(); if inserted { table.added.insert(id); } table.changed.insert(id); self.lifecycle.emit(LifecycleEvent::ComponentInserted { entity:id, component:type_id }); inserted }
    pub fn set_component<T: Any + Send + Sync>(&mut self, id: EntityId, component: T) -> bool { if !self.contains(id) { return false; } let type_id=TypeId::of::<T>(); let table=self.component_table::<T>(); let replaced=table.values.insert(id,component).is_some(); if !replaced { table.added.insert(id); self.lifecycle.emit(LifecycleEvent::ComponentInserted { entity:id, component:type_id }); } table.changed.insert(id); replaced }
    pub fn component<T: Any + Send + Sync>(&self, id: EntityId) -> Option<&T> { self.components.get(&TypeId::of::<T>())?.as_any().downcast_ref::<TypedComponentTable<T>>()?.values.get(&id) }
    pub fn component_mut<T: Any + Send + Sync>(&mut self, id: EntityId) -> Option<&mut T> { let table=self.components.get_mut(&TypeId::of::<T>())?.as_any_mut().downcast_mut::<TypedComponentTable<T>>()?; if !table.values.contains_key(&id) { return None; } table.changed.insert(id); table.values.get_mut(&id) }
    pub fn remove_component<T: Any + Send + Sync>(&mut self, id: EntityId) -> Option<T> { let type_id=TypeId::of::<T>(); let removed=self.components.get_mut(&type_id)?.as_any_mut().downcast_mut::<TypedComponentTable<T>>()?.values.remove(&id); if removed.is_some() { let table=self.components.get_mut(&type_id).unwrap().as_any_mut().downcast_mut::<TypedComponentTable<T>>().unwrap(); table.added.remove(&id); table.changed.remove(&id); self.lifecycle.emit(LifecycleEvent::ComponentRemoved { entity:id, component:type_id }); } removed }
    pub fn has_component<T: Any + Send + Sync>(&self, id: EntityId) -> bool { self.components.get(&TypeId::of::<T>()).is_some_and(|t| t.contains_entity(id)) }
    pub fn is_component_added<T: Any + Send + Sync>(&self, id: EntityId) -> bool { self.components.get(&TypeId::of::<T>()).is_some_and(|t| t.is_added(id)) }
    pub fn is_component_changed<T: Any + Send + Sync>(&self, id: EntityId) -> bool { self.components.get(&TypeId::of::<T>()).is_some_and(|t| t.is_changed(id)) }
    pub fn clear_change_tracking(&mut self) { for table in self.components.values_mut() { table.clear_changes(); } }
    pub fn lifecycle_hooks(&mut self) -> &mut LifecycleHooks { &mut self.lifecycle }
    pub fn lifecycle_events(&self) -> &[LifecycleEvent] { self.lifecycle.events() }
    pub fn drain_lifecycle_events(&mut self) -> Vec<LifecycleEvent> { self.lifecycle.drain_events() }
    pub fn insert_resource<T: Any + Send + Sync>(&mut self, resource:T)->Option<T>{self.resources.insert(resource)}
    pub fn resource<T: Any + Send + Sync>(&self)->Option<&T>{self.resources.get()}
    pub fn resource_mut<T: Any + Send + Sync>(&mut self)->Option<&mut T>{self.resources.get_mut()}
    pub fn remove_resource<T: Any + Send + Sync>(&mut self)->Option<T>{self.resources.remove()}
    pub fn has_resource<T: Any + Send + Sync>(&self)->bool{self.resources.contains()}
    pub fn resource_count(&self)->usize{self.resources.len()}
    pub fn set_parent(&mut self,child:EntityId,parent:Option<EntityId>)->bool{if !self.contains(child)||parent==Some(child)||parent.is_some_and(|p|!self.contains(p)){return false;}if let Some(p)=parent{if self.would_cycle(child,p){return false;}self.parents.insert(child,Parent(p));}else{self.parents.remove(&child);}true}
    fn would_cycle(&self,child:EntityId,proposed_parent:EntityId)->bool{let mut current=proposed_parent;for _ in 0..=self.parents.len(){if current==child{return true;}match self.parents.get(&current){Some(p)=>current=p.0,None=>return false}}true}
    pub fn parent(&self,child:EntityId)->Option<EntityId>{self.parents.get(&child).map(|p|p.0)}
    pub fn len(&self)->usize{self.transforms.len()}
    pub fn is_empty(&self)->bool{self.transforms.is_empty()}
    pub fn entities(&self)->Vec<EntityId>{let mut ids:Vec<_>=self.transforms.keys().copied().collect();ids.sort_by_key(|id|id.0);ids}
    pub fn iter_transforms(&self)->impl Iterator<Item=(EntityId,&Transform)>{self.entities().into_iter().filter_map(|id|self.transforms.get(&id).map(|t|(id,t)))}
    pub fn query1<A:Any+Send+Sync>(&self)->Vec<(EntityId,&A)>{self.entities().into_iter().filter_map(|id|self.component::<A>(id).map(|a|(id,a))).collect()}
    pub fn query2<A:Any+Send+Sync,B:Any+Send+Sync>(&self)->Vec<(EntityId,&A,&B)>{self.entities().into_iter().filter_map(|id|Some((id,self.component::<A>(id)?,self.component::<B>(id)?))).collect()}
    pub fn world_transform(&self,id:EntityId)->Option<Transform>{if !self.contains(id){return None;}let mut chain=Vec::new();let mut current=id;for _ in 0..=self.parents.len(){chain.push(current);match self.parents.get(&current){Some(p)=>current=p.0,None=>break}}if chain.len()>self.parents.len()+1{return None;}let mut result=*self.transforms.get(chain.last()?)?;for entity in chain.iter().rev().skip(1){result=combine(result,*self.transforms.get(entity)?);}Some(result)}
    pub fn query_world_transforms(&self)->Vec<(EntityId,Transform)>{self.entities().into_iter().filter_map(|id|self.world_transform(id).map(|t|(id,t))).collect()}
}

fn combine(parent:Transform,local:Transform)->Transform{Transform{translation:[parent.translation[0]+local.translation[0]*parent.scale[0],parent.translation[1]+local.translation[1]*parent.scale[1],parent.translation[2]+local.translation[2]*parent.scale[2]],rotation_xyzw:quat_mul(parent.rotation_xyzw,local.rotation_xyzw),scale:[parent.scale[0]*local.scale[0],parent.scale[1]*local.scale[1],parent.scale[2]*local.scale[2]]}}
fn quat_mul(a:[f32;4],b:[f32;4])->[f32;4]{[a[3]*b[0]+a[0]*b[3]+a[1]*b[2]-a[2]*b[1],a[3]*b[1]-a[0]*b[2]+a[1]*b[3]+a[2]*b[0],a[3]*b[2]+a[0]*b[1]-a[1]*b[0]+a[2]*b[3],a[3]*b[3]-a[0]*b[0]-a[1]*b[1]-a[2]*b[2]]}

pub struct TransformRenderPipeline<R>{pub world:World,pub renderer:R}
impl<R:Renderer> TransformRenderPipeline<R>{pub fn render(&mut self,frame:FrameId){self.renderer.begin_frame(frame);for(id,transform)in self.world.query_world_transforms(){self.renderer.submit(id,transform);}self.renderer.end_frame();}}

pub struct WorldEcsBridge{chunk_entities:HashMap<WorldChunkId,EntityId>}
impl Default for WorldEcsBridge{fn default()->Self{Self{chunk_entities:HashMap::new()}}}
impl WorldEcsBridge{pub fn new()->Self{Self::default()}pub fn entity_for_chunk(&self,chunk:WorldChunkId)->Option<EntityId>{self.chunk_entities.get(&chunk).copied()}pub fn sync(&mut self,world:&WorldStreamer,ecs:&mut World)->Vec<(WorldChunkId,EntityId)>{let mut loaded=Vec::new();for chunk in world.chunks(){if chunk.state!=ChunkState::Loaded{continue;}let entity=*self.chunk_entities.entry(chunk.id).or_insert_with(||EntityId(u64::MAX-chunk.id.0));let center=[(chunk.bounds.min[0]+chunk.bounds.max[0])*0.5,(chunk.bounds.min[1]+chunk.bounds.max[1])*0.5,(chunk.bounds.min[2]+chunk.bounds.max[2])*0.5];let transform=Transform{translation:center,..Default::default()};if !ecs.set_transform(entity,transform)&&!ecs.insert(entity,transform){continue;}loaded.push((chunk.id,entity));}let active:HashSet<_>=world.chunks().iter().filter(|c|c.state==ChunkState::Loaded).map(|c|c.id).collect();let stale:Vec<_>=self.chunk_entities.keys().copied().filter(|id|!active.contains(id)).collect();for chunk in stale{if let Some(entity)=self.chunk_entities.remove(&chunk){ecs.despawn(entity);}}loaded.sort_by_key(|(chunk,_)|chunk.0);loaded}}

#[cfg(test)]
mod tests{use super::*;#[derive(Debug,PartialEq)]struct Health(u32);#[test]fn change_tracking_works(){let mut w=World::new();let e=w.spawn(Transform::default());assert!(w.insert_component(e,Health(10)));assert!(w.is_component_added::<Health>(e));assert!(w.is_component_changed::<Health>(e));w.clear_change_tracking();*w.component_mut::<Health>(e).unwrap()=Health(5);assert!(!w.is_component_added::<Health>(e));assert!(w.is_component_changed::<Health>(e));}#[test]fn lifecycle_events_are_ordered(){let mut w=World::new();let e=w.spawn(Transform::default());w.set_component(e,Health(1));assert!(matches!(w.lifecycle_events()[0],LifecycleEvent::Spawned(id)if id==e));assert!(matches!(w.lifecycle_events()[1],LifecycleEvent::ComponentInserted{entity:id,..}if id==e));assert!(w.despawn(e));assert!(matches!(w.lifecycle_events()[2],LifecycleEvent::Despawned(id)if id==e));}#[test]fn executor_runs_in_dependency_order(){let mut w=World::new();let mut ex=SystemExecutor::new();ex.register(SystemDescriptor::new(SystemId(2),"b").after(SystemId(1)),|w|{w.insert_resource(2u32);}).unwrap();ex.register(SystemDescriptor::new(SystemId(1),"a"),|w|{w.insert_resource(1u32);}).unwrap();assert_eq!(ex.run(&mut w).unwrap(),vec![SystemId(1),SystemId(2)]);assert_eq!(w.resource::<u32>(),Some(&2));}#[test]fn hierarchy_resolves(){let mut w=World::new();let p=w.spawn(Transform{translation:[2.0,0.0,0.0],..Default::default()});let c=w.spawn(Transform{translation:[1.0,0.0,0.0],..Default::default()});assert!(w.set_parent(c,Some(p)));assert_eq!(w.world_transform(c).unwrap().translation[0],3.0);}}
