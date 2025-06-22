use crate::renderer::Renderer2D;

pub struct WgpuContext {
    clear_color: wgpu::Color,
    renderer: Renderer2D,
    screen_state: ScreenState,
    time: f32,
}

pub struct ScreenState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}
