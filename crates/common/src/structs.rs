#[derive(Debug, Clone)]
pub struct Shader(wgpu::ShaderModule);

#[derive(Debug, Clone)]
pub struct RenderPipeline(wgpu::RenderPipeline);

#[derive(Debug, Clone)]
pub struct PipelineLayout(wgpu::PipelineLayout);

#[derive(Debug, Clone)]
pub struct Buffer(wgpu::Buffer);

#[derive(Debug, Clone)]
pub struct Bindgroup(wgpu::BindGroup);

#[derive(Debug, Clone)]
pub struct BindgroupLayout(wgpu::BindGroupLayout);

#[derive(Debug, Clone)]
pub struct Texture {
    pub texture: wgpu::Texture,
    pub sampler: wgpu::Sampler,
    pub view: wgpu::TextureView,
}
