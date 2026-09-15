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
impl<T> SnapshotBuffer<T> { pub fn with_capacity(capacity:usize)->Self{Self{capacity:capacity.max(1),entries:Vec::new()}} pub fn push(&mut self,snapshot:Snapshot<T>){if self.entries.last().is_some_and(|s|snapshot.sequence<=s.sequence){return;}self.entries.push(snapshot);if self.entries.len()>self.capacity{self.entries.remove(0);}} pub fn latest(&self)->Option<&Snapshot<T>>{self.entries.last()} pub fn sample_pair(&self,tick:Tick)->Option<(&Snapshot<T>,&Snapshot<T>)>{self.entries.windows(2).find(|w|w[0].tick<=tick&&tick<=w[1].tick).map(|w|(&w[0],&w[1]))} pub fn len(&self)->usize{self.entries.len()} pub fn is_empty(&self)->bool{self.entries.is_empty()} }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplicationHeader { pub tick: Tick, pub sequence: u64, pub entity: NetworkEntity }
impl ReplicationHeader { pub fn encode(self)->[u8;24]{let mut b=[0u8;24];b[0..8].copy_from_slice(&self.tick.0.to_le_bytes());b[8..16].copy_from_slice(&self.sequence.to_le_bytes());b[16..24].copy_from_slice(&self.entity.0.to_le_bytes());b} pub fn decode(b:&[u8])->Option<Self>{if b.len()<24{return None;}Some(Self{tick:Tick(u64::from_le_bytes(b[0..8].try_into().ok()?)),sequence:u64::from_le_bytes(b[8..16].try_into().ok()?),entity:NetworkEntity(u64::from_le_bytes(b[16..24].try_into().ok()?))})} }
pub trait ReplicationTransport{fn send(&mut self,bytes:&[u8])->Result<(),String>;}
#[derive(Default)]
pub struct LoopbackTransport{packets:Vec<Vec<u8>>}
impl LoopbackTransport{pub fn drain(&mut self)->Vec<Vec<u8>>{std::mem::take(&mut self.packets)}}
impl ReplicationTransport for LoopbackTransport{fn send(&mut self,bytes:&[u8])->Result<(),String>{self.packets.push(bytes.to_vec());Ok(())}}
#[cfg(test)]mod tests{use super::*;#[test]fn buffer_is_bounded_and_ordered(){let mut b=SnapshotBuffer::with_capacity(2);b.push(Snapshot{tick:Tick(0),sequence:1,value:1});b.push(Snapshot{tick:Tick(1),sequence:2,value:2});b.push(Snapshot{tick:Tick(2),sequence:1,value:3});assert_eq!(b.len(),2);assert_eq!(b.latest().unwrap().value,2);}#[test]fn header_roundtrip(){let h=ReplicationHeader{tick:Tick(4),sequence:7,entity:NetworkEntity(9)};assert_eq!(ReplicationHeader::decode(&h.encode()),Some(h));}}
