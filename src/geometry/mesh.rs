use glam::{Mat4, UVec3, Vec2, Vec3};
use std::ops::{Add, Mul, Sub};

use crate::{
    texture::Texture,
    triangle::{vertex::Vertx, Tri},
};

pub struct Mesh {
    triangles: Vec<Tri>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            triangles: Vec::new(),
        }
    }

    pub fn get_triangles(&self) -> &Vec<Tri> {
        &self.triangles
    }

    pub fn add_section(&mut self, triangles: &mut [Tri]) {
        self.triangles.extend_from_slice(triangles);
    }

    pub fn from_vertices(vertices: &[Vertx]) -> Self {
        let mut tris: Vec<Tri> = Vec::new();
        for mut i in 0..vertices.len() {
            tris.push(Tri {
                vert_a: vertices[i],
                vert_b: vertices[i + 1],
                vert_c: vertices[i + 2],
            });

            if i + 3 < vertices.len() {
                i += 3;
            } else {
                break;
            }
        }

        Self { triangles: tris }
    }

    pub fn raster(
        &self,
        texture: Option<&Texture>,
        buffer: &mut Vec<u32>,
        z_buffer: &mut Vec<f32>,
        window_size: Vec2,
        mvp_mat: &Mat4,
    ) {
        for triangle in self.get_triangles() {
            triangle.raster(buffer, z_buffer, texture, window_size, mvp_mat);
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}
