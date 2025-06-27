use std::sync::Arc;

use crate::camera::CameraResource;

mod spritepass;
use common::traits::Renderer;
pub use spritepass::*;
use winit::window::Window;

pub struct Renderer2D {
    sprite_pass: SpritePass,
    //map_pass: TileMapPass,
    camera: CameraResource,
}

impl Renderer for Renderer2D {
    fn render(&self, device: &wgpu::Device, surface: &wgpu::Surface) {
        todo!()
    }
}
