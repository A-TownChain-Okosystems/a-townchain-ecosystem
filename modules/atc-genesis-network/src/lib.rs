#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NetworkEntity(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tick(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplicationMode { Authority, Predicted, Interpolated }
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot<T> { pub tick: Tick, pub sequence: u64, pub value: T }
#[derive(Default)]
pub struct SnapshotBuffer<T> { capacity: usize, entries: Vec<Snapshot<T>> }
impl<T> SnapshotBuffer<T> {
 pub fn with_capacity(capacity: usize)->Self{Self{capacity:capacity.max(1),entries:Vec::new()}}
 pub fn push(&mut self,s:Snapshot<T>){if self.entries.last().is_some_and(|x|s.sequence<=x.sequence){return;}self.entries.push(s);if self.entries.len()>self.capacity{self.entries.remove(0);}}
 pub fn latest(&self)->Option<&Snapshot<T>>{self.entries.last()}
 pub fn sample_pair(&self,tick:Tick)->Option<(&Snapshot<T>,&Snapshot<T>)>{self.entries.windows(2).find(|w|w[0].tick<=tick&&tick<=w[1].tick).map(|w|(&w[0],&w[1]))}
 pub fn len(&self)->usize{self.entries.len()}
 pub fn is_empty(&self)->bool{self.entries.is_empty()}
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ReplicationHeader{pub tick:Tick,pub sequence:u64,pub entity:NetworkEntity}
impl ReplicationHeader{pub const BYTES:usize=24;pub fn encode(self)->[u8;Self::BYTES]{let mut b=[0;Self::BYTES];b[0..8].copy_from_slice(&self.tick.0.to_le_bytes());b[8..16].copy_from_slice(&self.sequence.to_le_bytes());b[16..24].copy_from_slice(&self.entity.0.to_le_bytes());b}pub fn decode(b:&[u8])->Option<Self>{if b.len()<Self::BYTES{return None}Some(Self{tick:Tick(u64::from_le_bytes(b[0..8].try_into().ok()?)),sequence:u64::from_le_bytes(b[8..16].try_into().ok()?),entity:NetworkEntity(u64::from_le_bytes(b[16..24].try_into().ok()?))})}}
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ReplicatedState{pub entity:NetworkEntity,pub tick:Tick,pub position_mm:[i32;3],pub rotation_millirad:[i32;3]}
impl ReplicatedState{pub const BYTES:usize=40;pub fn encode(&self)->[u8;Self::BYTES]{let mut b=[0;Self::BYTES];b[0..8].copy_from_slice(&self.entity.0.to_le_bytes());b[8..16].copy_from_slice(&self.tick.0.to_le_bytes());for(i,v)in self.position_mm.iter().enumerate(){b[16+i*4..20+i*4].copy_from_slice(&v.to_le_bytes())}for(i,v)in self.rotation_millirad.iter().enumerate(){b[28+i*4..32+i*4].copy_from_slice(&v.to_le_bytes())}b}pub fn decode(b:&[u8])->Option<Self>{if b.len()<Self::BYTES{return None}let mut p=[0;3];let mut r=[0;3];for i in 0..3{p[i]=i32::from_le_bytes(b[16+i*4..20+i*4].try_into().ok()?);r[i]=i32::from_le_bytes(b[28+i*4..32+i*4].try_into().ok()?)}Some(Self{entity:NetworkEntity(u64::from_le_bytes(b[0..8].try_into().ok()?)),tick:Tick(u64::from_le_bytes(b[8..16].try_into().ok()?)),position_mm:p,rotation_millirad:r})}}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct InterpolatedTransform{pub position:[f32;3],pub rotation:[f32;3]}
impl InterpolatedTransform{pub fn between(a:&ReplicatedState,b:&ReplicatedState,alpha:f32)->Option<Self>{if a.entity!=b.entity||b.tick.0<a.tick.0{return None}let t=alpha.clamp(0.0,1.0);Some(Self{position:std::array::from_fn(|i|(a.position_mm[i]as f32+(b.position_mm[i]-a.position_mm[i])as f32*t)*0.001),rotation:std::array::from_fn(|i|(a.rotation_millirad[i]as f32+(b.rotation_millirad[i]-a.rotation_millirad[i])as f32*t)*0.001)})}}
pub struct PredictionBuffer{inputs:Vec<(Tick,[f32;3])>,capacity:usize}
impl PredictionBuffer{pub fn with_capacity(capacity:usize)->Self{Self{inputs:Vec::new(),capacity:capacity.max(1)}}pub fn push(&mut self,tick:Tick,input:[f32;3]){self.inputs.push((tick,input));if self.inputs.len()>self.capacity{self.inputs.remove(0)}}pub fn inputs_after(&self,tick:Tick)->impl Iterator<Item=(Tick,[f32;3])>+ '_{self.inputs.iter().copied().filter(move|(t,_)|t.0>tick.0)}pub fn len(&self)->usize{self.inputs.len()}}
pub trait ReplicationTransport{fn send(&mut self,bytes:&[u8])->Result<(),String>;}
#[derive(Default)]pub struct LoopbackTransport{packets:Vec<Vec<u8>>}
impl LoopbackTransport{pub fn drain(&mut self)->Vec<Vec<u8>>{std::mem::take(&mut self.packets)}}
impl ReplicationTransport for LoopbackTransport{fn send(&mut self,bytes:&[u8])->Result<(),String>{self.packets.push(bytes.to_vec());Ok(())}}
#[derive(Default)]pub struct Replicator<T:ReplicationTransport>{pub transport:T,sequence:u64,last_received_sequence:u64}
impl<T:ReplicationTransport>Replicator<T>{pub fn new(transport:T)->Self{Self{transport,sequence:0,last_received_sequence:0}}pub fn publish(&mut self,state:&ReplicatedState)->Result<u64,String>{self.sequence=self.sequence.saturating_add(1);let h=ReplicationHeader{tick:state.tick,sequence:self.sequence,entity:state.entity};let mut p=Vec::with_capacity(64);p.extend_from_slice(&h.encode());p.extend_from_slice(&state.encode());self.transport.send(&p)?;Ok(self.sequence)}pub fn decode_packet(bytes:&[u8])->Option<(ReplicationHeader,ReplicatedState)>{if bytes.len()<ReplicationHeader::BYTES+ReplicatedState::BYTES{return None}let header=ReplicationHeader::decode(&bytes[..ReplicationHeader::BYTES])?;let state=ReplicatedState::decode(&bytes[ReplicationHeader::BYTES..ReplicationHeader::BYTES+ReplicatedState::BYTES])?;if state.entity!=header.entity||state.tick!=header.tick{return None}Some((header,state))}pub fn accept_packet(&mut self,bytes:&[u8])->Option<ReplicatedState>{let (header,state)=Self::decode_packet(bytes)?;if header.sequence<=self.last_received_sequence{return None}self.last_received_sequence=header.sequence;Some(state)}pub fn sequence(&self)->u64{self.sequence}pub fn last_received_sequence(&self)->u64{self.last_received_sequence}}
#[cfg(test)]mod tests{use super::*;fn state()->ReplicatedState{ReplicatedState{entity:NetworkEntity(2),tick:Tick(8),position_mm:[1,-2,3],rotation_millirad:[4,5,-6]}}#[test]fn buffer(){let mut b=SnapshotBuffer::with_capacity(2);b.push(Snapshot{tick:Tick(0),sequence:1,value:1});b.push(Snapshot{tick:Tick(1),sequence:2,value:2});b.push(Snapshot{tick:Tick(2),sequence:1,value:3});assert_eq!(b.latest().unwrap().value,2)}#[test]fn header(){let h=ReplicationHeader{tick:Tick(4),sequence:7,entity:NetworkEntity(9)};assert_eq!(ReplicationHeader::decode(&h.encode()),Some(h))}#[test]fn state_roundtrip(){let s=state();assert_eq!(ReplicatedState::decode(&s.encode()),Some(s))}#[test]fn interpolation(){let a=ReplicatedState{entity:NetworkEntity(1),tick:Tick(1),position_mm:[0;3],rotation_millirad:[0;3]};let b=ReplicatedState{entity:NetworkEntity(1),tick:Tick(2),position_mm:[1000,2000,3000],rotation_millirad:[100,200,300]};let p=InterpolatedTransform::between(&a,&b,0.5).unwrap();assert_eq!(p.position,[0.5,1.0,1.5])}#[test]fn prediction(){let mut p=PredictionBuffer::with_capacity(4);p.push(Tick(1),[1.0,0.0,0.0]);p.push(Tick(2),[2.0,0.0,0.0]);assert_eq!(p.inputs_after(Tick(1)).count(),1)}#[test]fn packet_rejects_replay(){let mut r=Replicator::new(LoopbackTransport::default());let s=state();r.publish(&s).unwrap();let packet=r.transport.drain().pop().unwrap();assert!(r.accept_packet(&packet).is_some());assert!(r.accept_packet(&packet).is_none())}}
