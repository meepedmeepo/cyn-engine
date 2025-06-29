use std::sync::Arc;

use graphics::WgpuContext;

pub struct GraphicState<R: common::traits::Renderer> {
    pub graphics_context: WgpuContext,
    pub renderer: R,
    pub dimensions: DisplayDimensions,
}

///### Don't set either of the screen dimensions to zero else it will crash and burn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayDimensions(pub u32, pub u32);

pub trait AppEngine<GraphicState> {
    fn tick(&mut self, graphics_context: &GraphicState);

    fn init(&mut self);
}
