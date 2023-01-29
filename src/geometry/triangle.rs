#[path = "./vertex.rs"]
pub mod vertex;
use glam::Vec3Swizzles;
use vertex::Vertx;

use crate::{helper::{barycentric_coords, edge_function, index_to_coords, to_argb8}, texture::Texture};

pub struct Tri {
    pub vert_a: Vertx,
    pub vert_b: Vertx,
    pub vert_c: Vertx,
}

impl Tri {
    pub fn raster(&self, buffer: &mut Vec<u32>, z_buffer: &mut Vec<f32>, height: usize) {
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let coords = index_to_coords(i, height);
            let coords = glam::vec2(coords.0 as f32, coords.1 as f32);

            let tri_area = edge_function(
                self.vert_a.pos.xy(),
                self.vert_b.pos.xy(),
                self.vert_c.pos.xy(),
            );

            // subdivide triangle are using barycentric coordinates
            if let Some(bar_coords) = barycentric_coords(
                coords,
                self.vert_a.pos.xy(),
                self.vert_b.pos.xy(),
                self.vert_c.pos.xy(),
                tri_area,
            ) {
                let depth = bar_coords.x * self.vert_a.pos.z
                    + bar_coords.y * self.vert_b.pos.z
                    + bar_coords.z * self.vert_c.pos.z;

                if depth < z_buffer[i] {
                    z_buffer[i] = depth;

                    let color = bar_coords.x * self.vert_a.color
                        + bar_coords.y * self.vert_b.color
                        + bar_coords.z * self.vert_c.color;

                    *pixel = to_argb8(
                        255,
                        (color.x * 255.0) as u8,
                        (color.y * 255.0) as u8,
                        (color.z * 255.0) as u8,
                    );
                }
            }
        }
    }

    pub fn raster_textured(&self, buffer: &mut Vec<u32>, z_buffer: &mut Vec<f32>, texture: &Texture, height: usize) {
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let coords = index_to_coords(i, height);
            let coords = glam::vec2(coords.0 as f32, coords.1 as f32);

            let tri_area = edge_function(
                self.vert_a.pos.xy(),
                self.vert_b.pos.xy(),
                self.vert_c.pos.xy(),
            );

            // subdivide triangle are using barycentric coordinates
            if let Some(bar_coords) = barycentric_coords(
                coords,
                self.vert_a.pos.xy(),
                self.vert_b.pos.xy(),
                self.vert_c.pos.xy(),
                tri_area,
            ) {
                let depth = bar_coords.x * self.vert_a.pos.z
                    + bar_coords.y * self.vert_b.pos.z
                    + bar_coords.z * self.vert_c.pos.z;

                if depth < z_buffer[i] {
                    z_buffer[i] = depth;

                    let tex_coords = bar_coords.x * self.vert_a.uv + bar_coords.y * self.vert_b.uv + bar_coords.z * self.vert_c.uv;
                    let color = texture.sample_texture(tex_coords);

                    *pixel = color;
                }
            }
        }
    }
}
