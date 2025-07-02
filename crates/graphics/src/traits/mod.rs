use crate::{WgpuContext, camera::CameraResource};

pub trait RenderPass {
    fn run_render_pass(&self, ctx: &WgpuContext, camera: &CameraResource);
}
