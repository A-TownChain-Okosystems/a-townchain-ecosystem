use atc_genesis_platform::EntityId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SceneNode { pub id: EntityId, pub parent: Option<EntityId>, pub name: String }

#[derive(Default)]
pub struct SceneDocument { nodes: Vec<SceneNode>, selection: Option<EntityId>, undo: Vec<Vec<SceneNode>> }

impl SceneDocument {
    pub fn create_node(&mut self, id: EntityId, name: impl Into<String>, parent: Option<EntityId>) {
        self.undo.push(self.nodes.clone());
        self.nodes.push(SceneNode{id,parent,name:name.into()});
    }
    pub fn remove_node(&mut self, id: EntityId) -> bool {
        if let Some(pos)=self.nodes.iter().position(|n| n.id==id) { self.undo.push(self.nodes.clone()); self.nodes.remove(pos); if self.selection==Some(id){self.selection=None;} true } else { false }
    }
    pub fn select(&mut self, id: Option<EntityId>) { self.selection=id; }
    pub fn selected(&self) -> Option<EntityId> { self.selection }
    pub fn nodes(&self) -> &[SceneNode] { &self.nodes }
    pub fn undo(&mut self) -> bool { if let Some(state)=self.undo.pop(){self.nodes=state; true}else{false} }
}

#[cfg(test)]
mod tests { use super::*; #[test] fn scene_edits_are_undoable(){ let mut d=SceneDocument::default(); d.create_node(EntityId(1),"Root",None); assert_eq!(d.nodes().len(),1); assert!(d.undo()); assert!(d.nodes().is_empty()); } }
