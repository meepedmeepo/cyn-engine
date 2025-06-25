use crate::structs::TexVertex;

pub struct SpritePass {
    vertex_queue: Vec<TexVertex>,
    indices: Vec<u32>,
    instances: Vec<crate::Instance>,
    instance_buffer: wgpu::Buffer,
    render_pipeline_layout: wgpu::PipelineLayout,
}
