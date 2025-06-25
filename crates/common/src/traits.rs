pub trait Renderer {
    fn render(&self, device: &wgpu::Device, surface: &wgpu::Surface);
}
