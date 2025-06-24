use crate::{camera::Camera, structs::TexVertex};

pub struct Renderer2D {
    sprite_pass: SpritePass,
    map_pass: TileMapPass,
    camera: Camera,
    camera_uniform_buffer: wgpu::Buffer,
}

pub struct SpritePass {
    vertex_queue: Vec<TexVertex>,
    indices: Vec<u32>,
    instances: Vec<crate::Instance>,
    instance_buffer: wgpu::Buffer,
    render_pipeline_layout: wgpu::PipelineLayout,
}

pub struct TileMapPass {}
