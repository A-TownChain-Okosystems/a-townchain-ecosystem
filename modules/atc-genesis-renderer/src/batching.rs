use atc_genesis_platform::{AssetId, EntityId, Transform};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderItem {
    pub entity: EntityId,
    pub transform: Transform,
    pub mesh: AssetId,
    pub material: AssetId,
    pub texture: Option<AssetId>,
    pub instance_group: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RenderBatch {
    pub mesh: AssetId,
    pub material: AssetId,
    pub texture: Option<AssetId>,
    pub instances: Vec<EntityId>,
}

pub fn build_batches(items: &[RenderItem]) -> Vec<RenderBatch> {
    let mut sorted = items.to_vec();
    sorted.sort_by(|a, b| {
        (a.mesh, a.material, a.texture, a.instance_group, a.entity)
            .cmp(&(b.mesh, b.material, b.texture, b.instance_group, b.entity))
    });
    let mut batches = Vec::new();
    for item in sorted {
        match batches.last_mut() {
            Some(batch: &mut RenderBatch)
                if batch.mesh == item.mesh
                    && batch.material == item.material
                    && batch.texture == item.texture =>
            {
                batch.instances.push(item.entity);
            }
            _ => batches.push(RenderBatch {
                mesh: item.mesh,
                material: item.material,
                texture: item.texture,
                instances: vec![item.entity],
            }),
        }
    }
    batches
}

#[cfg(test)]
mod tests {
    use super::*;
    fn id(v: u128) -> AssetId { AssetId(v) }
    #[test]
    fn batches_share_mesh_material_texture() {
        let items = vec![
            RenderItem { entity: EntityId(2), transform: Transform::default(), mesh: id(1), material: id(2), texture: None, instance_group: None },
            RenderItem { entity: EntityId(1), transform: Transform::default(), mesh: id(1), material: id(2), texture: None, instance_group: None },
            RenderItem { entity: EntityId(3), transform: Transform::default(), mesh: id(1), material: id(9), texture: None, instance_group: None },
        ];
        let batches = build_batches(&items);
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0].instances, vec![EntityId(1), EntityId(2)]);
    }
}
