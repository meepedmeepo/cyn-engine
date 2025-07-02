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
    pub fn new() -> Self {
        AssetStorage {
            shaders: SlotMap::with_key(),
            textures: SlotMap::with_key(),
            render_pipelines: SlotMap::with_key(),
            pipeline_layouts: SlotMap::with_key(),
            buffers: SlotMap::with_key(),
            id_type_maps: HashMap::new(),
        }
    }

    pub fn get(&self, id: AssetId) -> AssetResult {
        let asset_type = self
            .id_type_maps
            .get(&id)
            .expect("AssetStorage Error: Asset type was never registered so unable to be fetched!");

        match asset_type {
            AssetType::Shader => AssetResult::Shader(self.shaders.get(id).cloned()),
            AssetType::RenderPipeline => {
                AssetResult::RenderPipeline(self.render_pipelines.get(id).cloned())
            }
            AssetType::PipelineLayout => {
                AssetResult::PipelineLayout(self.pipeline_layouts.get(id).cloned())
            }
            AssetType::Buffer => AssetResult::Buffer(self.buffers.get(id).cloned()),
            AssetType::Texture => AssetResult::Texture(self.textures.get(id).cloned()),
            _ => AssetResult::Invalid,
        }
    }

    pub fn insert(&mut self, asset: AssetInsertType) {
        let (asset_type, key) = match asset {
            AssetInsertType::Buffer(buf) => (AssetType::Buffer, self.buffers.insert(buf)),
            AssetInsertType::RenderPipeline(pipe) => (
                AssetType::RenderPipeline,
                self.render_pipelines.insert(pipe),
            ),
            AssetInsertType::PipelineLayout(layout) => (
                AssetType::PipelineLayout,
                self.pipeline_layouts.insert(layout),
            ),
            AssetInsertType::Shader(shader) => (AssetType::Shader, self.shaders.insert(shader)),
            AssetInsertType::Texture(tex) => (AssetType::Texture, self.textures.insert(tex)),
        };

        self.id_type_maps
            .insert(key, asset_type)
            .expect("Couldn't insert asset type into type map; Asset ID must already exist in it.");
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

#[derive(Debug, Clone)]
pub enum AssetInsertType {
    Shader(common::structs::Shader),
    RenderPipeline(common::structs::RenderPipeline),
    PipelineLayout(common::structs::PipelineLayout),
    Buffer(common::structs::Buffer),
    Texture(common::structs::Texture),
}
