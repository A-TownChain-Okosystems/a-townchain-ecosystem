use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildProfile { pub name: String, pub optimize: bool, pub debug_symbols: bool }

impl BuildProfile { pub fn debug() -> Self { Self{name:"debug".into(), optimize:false, debug_symbols:true} } pub fn release() -> Self { Self{name:"release".into(), optimize:true, debug_symbols:false} } }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageManifest { pub name: String, pub version: String, pub target: String, pub files: Vec<PathBuf> }

impl PackageManifest {
    pub fn new(name: impl Into<String>, version: impl Into<String>, target: impl Into<String>) -> Self { Self{name:name.into(),version:version.into(),target:target.into(),files:Vec::new()} }
    pub fn add_file(&mut self, path: impl AsRef<Path>) { self.files.push(path.as_ref().to_path_buf()); }
}
