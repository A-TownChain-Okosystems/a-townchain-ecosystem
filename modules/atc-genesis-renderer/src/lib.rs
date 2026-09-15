use atc_genesis_platform::{EntityId, FrameId, Renderer, Transform};
pub mod backend;
pub use backend::{BackendRenderer, GraphicsBackend, NullBackend, RenderBackend, RenderCapabilities};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera{pub position:[f32;3],pub forward:[f32;3],pub fov_y_radians:f32,pub near:f32,pub far:f32}
impl Default for Camera{fn default()->Self{Self{position:[0.0,0.0,0.0],forward:[0.0,0.0,-1.0],fov_y_radians:1.0472,near:0.1,far:1000.0}}}
impl Camera{pub fn visible_distance(&self,point:[f32;3])->bool{let dx=point[0]-self.position[0];let dy=point[1]-self.position[1];let dz=point[2]-self.position[2];let d=(dx*dx+dy*dy+dz*dz).sqrt();d>=self.near&&d<=self.far}}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawCommand{pub entity:EntityId,pub transform:Transform}
#[derive(Default)]
pub struct RenderGraph{commands:Vec<DrawCommand>}
impl RenderGraph{pub fn clear(&mut self){self.commands.clear()}pub fn push(&mut self,c:DrawCommand){self.commands.push(c)}pub fn commands(&self)->&[DrawCommand]{&self.commands}pub fn sort_deterministic(&mut self){self.commands.sort_by_key(|c|c.entity.0)}}

#[derive(Default)]
pub struct NullRenderer{frame:Option<FrameId>,submitted:usize,pub graph:RenderGraph}
impl NullRenderer{pub fn submitted_count(&self)->usize{self.submitted}}
impl Renderer for NullRenderer{fn begin_frame(&mut self,frame:FrameId){self.frame=Some(frame);self.submitted=0;self.graph.clear()}fn submit(&mut self,entity:EntityId,transform:Transform){if self.frame.is_some(){self.submitted+=1;self.graph.push(DrawCommand{entity,transform})}}fn end_frame(&mut self){self.graph.sort_deterministic();self.frame=None}}
#[cfg(test)]mod tests{use super::*;#[test]fn graph_orders_entities(){let mut r=NullRenderer::default();r.begin_frame(FrameId(1));r.submit(EntityId(3),Transform::default());r.submit(EntityId(1),Transform::default());r.end_frame();assert_eq!(r.graph.commands()[0].entity,EntityId(1));}}
