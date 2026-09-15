use atc_genesis_platform::{AssetId, AssetKind};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetMetadata { pub id: AssetId, pub kind: AssetKind, pub source: String, pub dependencies: Vec<AssetId> }

#[derive(Default)]
pub struct AssetRegistry { assets: HashMap<AssetId, AssetMetadata> }
impl AssetRegistry {
    pub fn register(&mut self, asset: AssetMetadata) { self.assets.insert(asset.id, asset); }
    pub fn get(&self, id: AssetId) -> Option<&AssetMetadata> { self.assets.get(&id) }
    pub fn dependencies(&self, id: AssetId) -> Option<&[AssetId]> { self.assets.get(&id).map(|a| a.dependencies.as_slice()) }
    pub fn len(&self) -> usize { self.assets.len() }
    pub fn is_empty(&self) -> bool { self.assets.is_empty() }
}
