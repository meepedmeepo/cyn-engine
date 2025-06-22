use crate::structs::TexVertex;

pub struct Renderer2D {
    sprite_pass: SpritePass,
    map_pass: TileMapPass,
}

pub struct SpritePass {
    vertex_queue: Vec<TexVertex>,
    indices: Vec<u32>,
}

pub struct TileMapPass {}
