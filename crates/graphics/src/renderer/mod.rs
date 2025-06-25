use crate::{camera::CameraResource, structs::TexVertex};

mod spritepass;
pub use spritepass::*;

pub struct Renderer2D {
    sprite_pass: SpritePass,
    //map_pass: TileMapPass,
    camera: CameraResource,
}
