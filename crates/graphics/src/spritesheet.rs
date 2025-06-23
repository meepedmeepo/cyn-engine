use std::sync::Arc;

use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone)]
pub struct SpriteSheet {
    texture: Arc<crate::Texture>,
    cols: u16,
    rows: u16,
    sprite_dimensions: cgmath::Vector2<u16>,
}

impl SpriteSheet {
    pub fn new(
        texture: Arc<crate::Texture>,
        cols: u16,
        rows: u16,
        sprite_dimensions: cgmath::Vector2<u16>,
    ) -> Self {
        SpriteSheet {
            texture,
            rows,
            cols,
            sprite_dimensions,
        }
    }

    pub fn to_raw(&self, col: u16, row: u16) -> SpriteInstance {
        SpriteInstance::new(col, row, self.sprite_dimensions.into())
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct SpriteInstance {
    col: u16,
    row: u16,
    sprite_dimensions: [u16; 2],
}

impl SpriteInstance {
    pub fn new(col: u16, row: u16, sprite_dimensions: [u16; 2]) -> Self {
        SpriteInstance {
            col,
            row,
            sprite_dimensions,
        }
    }
}
