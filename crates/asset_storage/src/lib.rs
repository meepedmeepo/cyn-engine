use std::collections::HashMap;

use slotmap::{DefaultKey, SlotMap, new_key_type};

pub mod store;

new_key_type! {pub struct AssetId;}

pub struct AssetStorage {
    pub(crate) shaders: SlotMap<AssetId, common::structs::Shader>,
    pub(crate) textures: SlotMap<AssetId, common::structs::Texture>,
    pub(crate) render_pipelines: SlotMap<AssetId, common::structs::RenderPipeline>,
    pub(crate) pipeline_layouts: SlotMap<AssetId, common::structs::PipelineLayout>,
    pub(crate) buffers: SlotMap<AssetId, common::structs::Buffer>,
    pub(crate) id_type_maps: HashMap<AssetId, AssetType>,
}

impl AssetStorage {
    pub fn get(&self, id: AssetId) -> AssetResult {
        let asset_type = **self
            .id_type_maps
            .get(&id)
            .get_or_insert(&AssetType::Invalid);

        match asset_type {
            AssetType::Shader => AssetResult::Shader(self.shaders.get(id).cloned()),
            AssetType::RenderPipeline => {
                AssetResult::RenderPipeline(self.render_pipelines.get(id).cloned())
            }
            _ => AssetResult::Invalid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    Shader,
    RenderPipeline,
    PipelineLayout,
    Buffer,
    Texture,
    Invalid,
}

#[derive(Debug, Clone)]
pub enum AssetResult {
    Shader(Option<common::structs::Shader>),
    RenderPipeline(Option<common::structs::RenderPipeline>),
    PipelineLayout(Option<common::structs::PipelineLayout>),
    Buffer(Option<common::structs::Buffer>),
    Texture(Option<common::structs::Texture>),
    Invalid,
}
