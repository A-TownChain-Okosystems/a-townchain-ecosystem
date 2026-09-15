use atc_genesis_platform::AssetId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldBounds {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl WorldBounds {
    pub fn contains(&self, p: [f32; 3]) -> bool {
        (0..3).all(|i| p[i] >= self.min[i] && p[i] <= self.max[i])
    }
    pub fn distance_squared(&self, p: [f32; 3]) -> f32 {
        let mut d = 0.0;
        for i in 0..3 {
            let delta = if p[i] < self.min[i] { self.min[i] - p[i] } else if p[i] > self.max[i] { p[i] - self.max[i] } else { 0.0 };
            d += delta * delta;
        }
        d
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorldChunkId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkState { Unloaded, Loading, Loaded }

#[derive(Clone, Debug, PartialEq)]
pub struct WorldChunk {
    pub id: WorldChunkId,
    pub asset: AssetId,
    pub bounds: WorldBounds,
    pub state: ChunkState,
}

#[derive(Default)]
pub struct WorldStreamer {
    chunks: Vec<WorldChunk>,
}

impl WorldStreamer {
    pub fn add_chunk(&mut self, chunk: WorldChunk) -> Result<(), &'static str> {
        if self.chunks.iter().any(|c| c.id == chunk.id) { return Err("duplicate chunk id"); }
        if chunk.bounds.min.iter().zip(chunk.bounds.max.iter()).any(|(a,b)| a > b) { return Err("invalid chunk bounds"); }
        self.chunks.push(chunk);
        self.chunks.sort_by_key(|c| c.id);
        Ok(())
    }
    pub fn chunks(&self) -> &[WorldChunk] { &self.chunks }
    pub fn loaded(&self) -> impl Iterator<Item=&WorldChunk> { self.chunks.iter().filter(|c| c.state == ChunkState::Loaded) }
    pub fn update(&mut self, position: [f32; 3], load_distance: f32, unload_distance: f32, budget: usize) -> Vec<WorldChunkId> {
        let load2 = load_distance.max(0.0).powi(2);
        let unload2 = unload_distance.max(load_distance).powi(2);
        let mut changed = Vec::new();
        for chunk in &mut self.chunks {
            let d2 = chunk.bounds.distance_squared(position);
            if chunk.state == ChunkState::Loaded && d2 > unload2 {
                chunk.state = ChunkState::Unloaded;
                changed.push(chunk.id);
            }
        }
        let mut candidates: Vec<(f32, WorldChunkId)> = self.chunks.iter().filter(|c| c.state == ChunkState::Unloaded).map(|c| (c.bounds.distance_squared(position), c.id)).filter(|(d,_)| *d <= load2).collect();
        candidates.sort_by(|a,b| a.0.total_cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
        for (_, id) in candidates.into_iter().take(budget) {
            if let Some(c) = self.chunks.iter_mut().find(|c| c.id == id) { c.state = ChunkState::Loaded; changed.push(id); }
        }
        changed.sort();
        changed.dedup();
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn chunk(id: u64, x: f32) -> WorldChunk { WorldChunk { id: WorldChunkId(id), asset: AssetId(id as u128), bounds: WorldBounds { min: [x,0.0,0.0], max: [x+10.0,10.0,10.0] }, state: ChunkState::Unloaded } }
    #[test] fn deterministic_budget_prefers_nearest() { let mut s=WorldStreamer::default(); s.add_chunk(chunk(2,20.0)).unwrap(); s.add_chunk(chunk(1,0.0)).unwrap(); assert_eq!(s.update([5.0,5.0,5.0],30.0,40.0,1),vec![WorldChunkId(1)]); }
    #[test] fn unloads_far_chunks() { let mut s=WorldStreamer::default(); let mut c=chunk(1,0.0); c.state=ChunkState::Loaded; s.add_chunk(c).unwrap(); assert_eq!(s.update([100.0,0.0,0.0],10.0,20.0,0),vec![WorldChunkId(1)]); assert_eq!(s.chunks()[0].state,ChunkState::Unloaded); }
}
