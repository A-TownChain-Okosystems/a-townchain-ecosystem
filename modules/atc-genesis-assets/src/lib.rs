use atc_genesis_platform::{AssetId, AssetKind, AssetStore};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AssetRecord { pub id: AssetId, pub kind: AssetKind }

#[derive(Default)]
pub struct InMemoryAssetStore { assets: Vec<AssetRecord> }
impl InMemoryAssetStore {
    pub fn register(&mut self,id:AssetId,kind:AssetKind){ if let Some(a)=self.assets.iter_mut().find(|a|a.id==id){a.kind=kind;}else{self.assets.push(AssetRecord{id,kind});} }
}
impl AssetStore for InMemoryAssetStore { fn contains(&self,id:AssetId)->bool{self.assets.iter().any(|a|a.id==id)} fn kind(&self,id:AssetId)->Option<AssetKind>{self.assets.iter().find(|a|a.id==id).map(|a|a.kind)} }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportRecord { pub source: String, pub kind: AssetKind, pub content_hash: u64, pub cooked: bool, pub dependencies: Vec<AssetId> }

#[derive(Default)]
pub struct AssetPipeline { records: HashMap<AssetId, ImportRecord> }
impl AssetPipeline {
    pub fn new() -> Self { Self::default() }
    pub fn import(&mut self,id:AssetId,source:impl Into<String>,kind:AssetKind,bytes:&[u8],dependencies:Vec<AssetId>)->u64 {
        let hash=fnv1a64(bytes);
        self.records.insert(id,ImportRecord{source:source.into(),kind,content_hash:hash,cooked:false,dependencies}); hash
    }
    pub fn cook(&mut self,id:AssetId)->bool { if let Some(r)=self.records.get_mut(&id){r.cooked=true;true}else{false} }
    pub fn get(&self,id:AssetId)->Option<&ImportRecord>{self.records.get(&id)}
    pub fn uncooked(&self)->Vec<AssetId>{self.records.iter().filter_map(|(id,r)|(!r.cooked).then_some(*id)).collect()}
    pub fn validate_dependencies(&self,id:AssetId)->bool { self.records.get(&id).map_or(false,|r|r.dependencies.iter().all(|d|self.records.contains_key(d))) }
}

fn fnv1a64(bytes:&[u8])->u64 { let mut h=0xcbf29ce484222325u64; for b in bytes {h^=*b as u64;h=h.wrapping_mul(0x100000001b3);} h }

#[cfg(test)]
mod tests { use super::*; #[test] fn import_cook_hash(){let mut p=AssetPipeline::new();let id=AssetId(1);let h=p.import(id,"mesh.gltf",AssetKind::Mesh,b"mesh",vec![]);assert_ne!(h,0);assert_eq!(p.uncooked(),vec![id]);assert!(p.cook(id));assert!(p.get(id).unwrap().cooked);} }
