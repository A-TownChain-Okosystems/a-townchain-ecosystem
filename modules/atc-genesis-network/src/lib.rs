#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NetworkEntity(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tick(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplicationMode { Authority, Predicted, Interpolated }

#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot<T> { pub tick: Tick, pub sequence: u64, pub value: T }

#[derive(Default)]
pub struct SnapshotBuffer<T> { capacity: usize, entries: Vec<Snapshot<T>> }

impl<T> SnapshotBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self { Self { capacity: capacity.max(1), entries: Vec::new() } }
    pub fn push(&mut self, snapshot: Snapshot<T>) {
        self.entries.push(snapshot);
        if self.entries.len() > self.capacity { self.entries.remove(0); }
    }
    pub fn latest(&self) -> Option<&Snapshot<T>> { self.entries.last() }
    pub fn len(&self) -> usize { self.entries.len() }
}

#[cfg(test)]
mod tests { use super::*; #[test] fn buffer_is_bounded() { let mut b=SnapshotBuffer::with_capacity(2); for i in 0..3 { b.push(Snapshot{tick:Tick(i),sequence:i,value:i}); } assert_eq!(b.len(),2); assert_eq!(b.latest().unwrap().value,2); } }
