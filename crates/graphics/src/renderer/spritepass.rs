use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Vector4};

use crate::{camera::CameraResource, structs::TexVertex, traits::RenderPass, WgpuContext};

pub struct SpritePass {
    vertex_queue: Vec<TexVertex>,
    indices: Vec<u32>,
    instances: Vec<SpriteInstance>,
    instance_buffer: wgpu::Buffer,
    render_pipeline_layout: wgpu::RenderPipeline,
}
impl SpritePass {
    pub fn new() -> Self {
        SpritePass {

        }

        todo!()
    }
}

impl RenderPass for SpritePass {
    fn run_render_pass(&self, ctx: &WgpuContext, camera: &CameraResource) {
        todo!()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct SpriteInstance {
    pub position: [[f32; 4]; 4],
    pub sprite_clip: SpriteClip,
}

impl SpriteInstance {
    pub fn new(pos_vec: cgmath::Vector3<f32>, sprite_clip: SpriteClip) -> Self {
        SpriteInstance {
            position: Matrix4::from_translation(pos_vec).into(),
            sprite_clip,
        }
    }

    pub fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SpriteInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                //Matrix4<f32>
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
                //Vector4<u32> - Sprite clip
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[u32; 16]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Uint32x4,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct SpriteClip {
    pub row: u32,
    pub col: u32,
    pub clip_width: u32,
    pub clip_height: u32,
}
