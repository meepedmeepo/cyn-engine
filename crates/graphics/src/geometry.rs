use cgmath::{Vector2, Vector3};

use crate::structs::TexVertex;

pub struct Quad {
    pub vertices: [TexVertex; 4],
    pub indices: [u32; 6],
}

impl Quad {
    //Creates a default quad the size of the entire screen.
    pub fn new() -> Self {
        let vertices: [TexVertex; 4] = [
            TexVertex::new([-1.0, -1.0], [0.0, 1.0]),
            TexVertex::new([1.0, -1.0], [1.0, 1.0]),
            TexVertex::new([-1.0, 1.0], [0., 0.]),
            TexVertex::new([1.0, 1.0], [1.0, 0.0]),
        ];

        #[rustfmt::skip]
        let indices: [u32; 6]= [
                                0, 1, 2,
                                1, 3, 2
                                ];

        Self { vertices, indices }
    }

    //Evenly scales the size of quad.
    pub fn scale_quad(&mut self, scale: f32) {
        if scale <= 0. {
            panic!("Quad scale must be greater than 0");
        }

        for v in self.vertices.iter_mut() {
            v.position[0] *= scale;
            v.position[1] *= scale;
        }
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        for tex in self.vertices.iter_mut() {
            tex.position[0] += translation.x;
            tex.position[1] += translation.y;
            tex.position[2] += translation.z;
        }
    }
}
