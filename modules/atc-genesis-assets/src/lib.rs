use atc_genesis_platform::{AssetId, AssetKind, AssetStore};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AssetRecord {
    pub id: AssetId,
    pub kind: AssetKind,
}

#[derive(Default)]
pub struct InMemoryAssetStore {
    assets: Vec<AssetRecord>,
}

impl InMemoryAssetStore {
    pub fn register(&mut self, id: AssetId, kind: AssetKind) {
        if let Some(existing) = self.assets.iter_mut().find(|asset| asset.id == id) {
            existing.kind = kind;
        } else {
            self.assets.push(AssetRecord { id, kind });
        }
    }
}

impl AssetStore for InMemoryAssetStore {
    fn contains(&self, id: AssetId) -> bool { self.assets.iter().any(|asset| asset.id == id) }
    fn kind(&self, id: AssetId) -> Option<AssetKind> { self.assets.iter().find(|asset| asset.id == id).map(|asset| asset.kind) }
}
