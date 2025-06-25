pub struct AppState<R: common::traits::Renderer> {
    pub graphics_context: common::WgpuContext,
    pub renderer: R,
    pub dimensions: DisplayDimensions,
}

///### Don't set either of the screen dimensions to zero else it will crash and burn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayDimensions(pub u32, pub u32);
