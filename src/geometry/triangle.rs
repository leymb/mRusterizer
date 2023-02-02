#[path = "./vertex.rs"]
pub mod vertex;
use glam::{Mat4, Vec2, Vec3, Vec3Swizzles, Vec4};
use vertex::Vertx;

use crate::{
    helper::{barycentric_coords, edge_function, index_to_coords, map_to_range, to_argb8},
    texture::Texture,
};

#[derive(Clone)]
pub struct Tri {
    pub vert_a: Vertx,
    pub vert_b: Vertx,
    pub vert_c: Vertx,
}

impl Tri {
    pub fn new(vertices: [Vertx; 3]) -> Self {
        Self {
            vert_a: (vertices[0]),
            vert_b: (vertices[1]),
            vert_c: (vertices[2]),
        }
    }

    pub fn raster(
        &self,
        buffer: &mut Vec<u32>,
        z_buffer: &mut Vec<f32>,
        texture: Option<&Texture>,
        viewport_size: Vec2,
        mvp_mat: &Mat4,
    ) {
        // calculate clipping matrices
        let clip0 = *mvp_mat * Vec4::from((self.vert_a.pos, 1.0));
        let clip1 = *mvp_mat * Vec4::from((self.vert_b.pos, 1.0));
        let clip2 = *mvp_mat * Vec4::from((self.vert_c.pos, 1.0));

        // perspective correction
        let rec0 = 1.0 / clip0.w;
        let rec1 = 1.0 / clip1.w;
        let rec2 = 1.0 / clip2.w;

        let uv0 = self.vert_a.uv * rec0;
        let uv1 = self.vert_b.uv * rec1;
        let uv2 = self.vert_c.uv * rec2;

        let color_vert_a = self.vert_a.color * rec0;
        let color_vert_b = self.vert_b.color * rec1;
        let color_vert_c = self.vert_c.color * rec2;

        let ndc0 = clip0 * rec0;
        let ndc1 = clip1 * rec1;
        let ndc2 = clip2 * rec2;

        // remap screen coords to window
        let sc0 = glam::vec2(
            map_to_range(ndc0.x, -1.0, 1.0, 0.0, viewport_size.x),
            map_to_range(ndc0.y, -1.0, 1.0, 0.0, viewport_size.y),
        );
        let sc1 = glam::vec2(
            map_to_range(ndc1.x, -1.0, 1.0, 0.0, viewport_size.x),
            map_to_range(ndc1.y, -1.0, 1.0, 0.0, viewport_size.y),
        );
        let sc2 = glam::vec2(
            map_to_range(ndc2.x, -1.0, 1.0, 0.0, viewport_size.x),
            map_to_range(ndc2.y, -1.0, 1.0, 0.0, viewport_size.y),
        );

        for (i, pixel) in buffer.iter_mut().enumerate() {
            let coords = index_to_coords(i, viewport_size.x as usize);
            let coords = glam::vec2(coords.0 as f32, coords.1 as f32) + 0.5;

            let tri_area = edge_function(sc0, sc1, sc2);

            // subdivide triangle are using barycentric coordinates
            if let Some(bar_coords) = barycentric_coords(coords, sc0, sc1, sc2, tri_area) {
                let perspective_correction =
                    bar_coords.x * rec0 + bar_coords.y * rec1 + bar_coords.z * rec2;
                let perspective_correction = 1.0 / perspective_correction;

                // corrected depth
                let depth = bar_coords.x * ndc0.z + bar_coords.y * ndc1.z + bar_coords.z * ndc2.z;

                if depth < z_buffer[i] {
                    z_buffer[i] = depth;

                    let vert_color = bar_coords.x * color_vert_a
                        + bar_coords.y * color_vert_b
                        + bar_coords.z * color_vert_c;
                    let vert_color = vert_color * perspective_correction;

                    let mut color = to_argb8(
                        255,
                        (vert_color.x * 255.0) as u8,
                        (vert_color.y * 255.0) as u8,
                        (vert_color.z * 255.0) as u8,
                    );

                    if let Some(texture) = texture {
                        let tex_coords =
                            bar_coords.x * uv0 + bar_coords.y * uv1 + bar_coords.z * uv2;
                        let tex_coords = tex_coords * perspective_correction;

                        color = texture.sample_texture(tex_coords);
                    }

                    *pixel = color;
                }
            }
        }
    }
}
