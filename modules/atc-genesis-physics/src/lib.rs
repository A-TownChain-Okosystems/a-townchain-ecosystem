use atc_genesis_platform::{EntityId, PhysicsWorld};
mod collision;
pub use collision::{Aabb, Collider, CollisionWorld};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RigidBody { pub entity: EntityId, pub position: [f32;3], pub velocity: [f32;3], pub mass: f32, pub dynamic: bool }
impl RigidBody { pub fn dynamic(entity:EntityId,position:[f32;3],mass:f32)->Self{Self{entity,position,velocity:[0.0;3],mass:mass.max(0.0001),dynamic:true}} }
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhysicsConfig { pub fixed_dt:f32, pub gravity:[f32;3], pub max_substeps:u32 }
impl Default for PhysicsConfig{fn default()->Self{Self{fixed_dt:1.0/60.0,gravity:[0.0,-9.81,0.0],max_substeps:8}}}
#[derive(Default)]
pub struct PhysicsSimulation{pub config:PhysicsConfig,accumulator:f32,bodies:Vec<RigidBody>,collisions:CollisionWorld}
impl PhysicsSimulation{pub fn new(config:PhysicsConfig)->Self{Self{config:PhysicsConfig{fixed_dt:config.fixed_dt.max(1e-6),..config},..Self::default()}}pub fn add_body(&mut self,body:RigidBody){self.bodies.push(body)}pub fn bodies(&self)->&[RigidBody]{&self.bodies}pub fn body(&self,entity:EntityId)->Option<&RigidBody>{self.bodies.iter().find(|b|b.entity==entity)}pub fn step_fixed(&mut self){let dt=self.config.fixed_dt;for body in &mut self.bodies{if body.dynamic{for i in 0..3{body.velocity[i]+=self.config.gravity[i]*dt;body.position[i]+=body.velocity[i]*dt;}}}}pub fn advance(&mut self,dt_seconds:f32)->u32{self.accumulator+=dt_seconds.clamp(0.0,1.0);let mut steps=0;while self.accumulator>=self.config.fixed_dt&&steps<self.config.max_substeps{self.step_fixed();self.accumulator-=self.config.fixed_dt;steps+=1;}steps}pub fn add_collider(&mut self,collider:Collider){self.collisions.add(collider)}pub fn query(&self,bounds:Aabb)->Vec<EntityId>{self.collisions.query_aabb(bounds)}}
impl PhysicsWorld for PhysicsSimulation{fn step(&mut self,dt_seconds:f32){let _=self.advance(dt_seconds);}fn raycast(&self,origin:[f32;3],direction:[f32;3],max_distance:f32)->Option<EntityId>{self.collisions.raycast(origin,direction,max_distance)}}
#[cfg(test)]mod tests{use super::*;#[test]fn fixed_step_applies_gravity(){let mut sim=PhysicsSimulation::new(PhysicsConfig::default());sim.add_body(RigidBody::dynamic(EntityId(1),[0.0,10.0,0.0],1.0));assert_eq!(sim.advance(1.0/60.0),1);assert!(sim.body(EntityId(1)).unwrap().position[1]<10.0);}}
