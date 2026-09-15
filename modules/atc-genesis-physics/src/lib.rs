use atc_genesis_platform::{EntityId,PhysicsWorld};
mod collision;
pub use collision::{Aabb,Collider,CollisionWorld};
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct RigidBody{pub entity:EntityId,pub position:[f32;3],pub velocity:[f32;3],pub mass:f32,pub dynamic:bool,pub restitution:f32}
impl RigidBody{pub fn dynamic(entity:EntityId,position:[f32;3],mass:f32)->Self{Self{entity,position,velocity:[0.0;3],mass:mass.max(0.0001),dynamic:true,restitution:0.0}}}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct PhysicsConfig{pub fixed_dt:f32,pub gravity:[f32;3],pub max_substeps:u32,pub floor_y:Option<f32>}
impl Default for PhysicsConfig{fn default()->Self{Self{fixed_dt:1.0/60.0,gravity:[0.0,-9.81,0.0],max_substeps:8,floor_y:None}}}
#[derive(Default)]
pub struct PhysicsSimulation{pub config:PhysicsConfig,accumulator:f32,bodies:Vec<RigidBody>,collisions:CollisionWorld}
impl PhysicsSimulation{
 pub fn new(config:PhysicsConfig)->Self{Self{config:PhysicsConfig{fixed_dt:config.fixed_dt.max(1e-6),max_substeps:config.max_substeps.max(1),..config},..Self::default()}}
 pub fn add_body(&mut self,body:RigidBody){self.bodies.push(body)}
 pub fn remove_body(&mut self,entity:EntityId)->bool{if let Some(i)=self.bodies.iter().position(|b|b.entity==entity){self.bodies.remove(i);self.collisions.remove(entity);true}else{false}}
 pub fn bodies(&self)->&[RigidBody]{&self.bodies}
 pub fn body(&self,entity:EntityId)->Option<&RigidBody>{self.bodies.iter().find(|b|b.entity==entity)}
 pub fn body_mut(&mut self,entity:EntityId)->Option<&mut RigidBody>{self.bodies.iter_mut().find(|b|b.entity==entity)}
 pub fn step_fixed(&mut self){let dt=self.config.fixed_dt;for body in &mut self.bodies{if !body.dynamic{continue}for i in 0..3{body.velocity[i]+=self.config.gravity[i]*dt;body.position[i]+=body.velocity[i]*dt;}if let Some(floor)=self.config.floor_y{if body.position[1]<floor{body.position[1]=floor;if body.velocity[1]<0.0{body.velocity[1]*=-body.restitution;}}}}}
 pub fn advance(&mut self,dt:f32)->u32{self.accumulator+=dt.clamp(0.0,1.0);let mut steps=0;while self.accumulator>=self.config.fixed_dt&&steps<self.config.max_substeps{self.step_fixed();self.accumulator-=self.config.fixed_dt;steps+=1;}steps}
 pub fn add_collider(&mut self,collider:Collider){self.collisions.add(collider)}
 pub fn query(&self,bounds:Aabb)->Vec<EntityId>{self.collisions.query_aabb(bounds)}
 pub fn interpolation_alpha(&self)->f32{(self.accumulator/self.config.fixed_dt).clamp(0.0,1.0)}
}
impl PhysicsWorld for PhysicsSimulation{fn step(&mut self,dt:f32){let _=self.advance(dt)}fn raycast(&self,origin:[f32;3],direction:[f32;3],max_distance:f32)->Option<EntityId>{self.collisions.raycast(origin,direction,max_distance)}}
#[cfg(test)]mod tests{use super::*;#[test]fn fixed_step_applies_gravity(){let mut sim=PhysicsSimulation::new(PhysicsConfig::default());sim.add_body(RigidBody::dynamic(EntityId(1),[0.0,10.0,0.0],1.0));assert_eq!(sim.advance(1.0/60.0),1);assert!(sim.body(EntityId(1)).unwrap().position[1]<10.0);}#[test]fn floor_stops_body(){let mut c=PhysicsConfig::default();c.gravity=[0.0,-10.0,0.0];c.floor_y=Some(0.0);let mut s=PhysicsSimulation::new(c);s.add_body(RigidBody::dynamic(EntityId(1),[0.0,0.01,0.0],1.0));for _ in 0..10{s.advance(1.0/60.0);}assert!(s.body(EntityId(1)).unwrap().position[1]>=0.0);}}
