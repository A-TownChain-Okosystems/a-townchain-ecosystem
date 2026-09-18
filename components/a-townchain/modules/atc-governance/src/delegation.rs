use std::collections::{BTreeMap,BTreeSet};

#[derive(Default)]
pub struct Delegation{delegations:BTreeMap<String,String>}
impl Delegation{
 pub fn new()->Self{Self::default()}
 pub fn delegate(&mut self,from:&str,to:&str)->Result<(),String>{if from==to{return Err("Cannot delegate to self".into())}if self.would_create_cycle(from,to){return Err("Cycle detected".into())}self.delegations.insert(from.into(),to.into());Ok(())}
 pub fn undelegate(&mut self,from:&str){self.delegations.remove(from);}
 pub fn get_delegate(&self,from:&str)->Option<&String>{self.delegations.get(from)}
 fn would_create_cycle(&self,from:&str,to:&str)->bool{let mut current=to;let mut seen=BTreeSet::new();loop{if current==from{return true}if !seen.insert(current.to_owned()){return true}match self.delegations.get(current){Some(next)=>current=next,None=>return false}}}
 pub fn resolve(&self,voter:&str)->String{let mut current=voter;let mut seen=BTreeSet::new();while let Some(next)=self.delegations.get(current){if !seen.insert(current.to_owned()){break}current=next;}current.into()}
}
