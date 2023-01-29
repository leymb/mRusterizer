use std::path::Path;

use glam::Vec2;

use crate::helper::{coords_to_index, to_argb8};

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
    pub depth: usize,
}

impl Texture {
    pub fn load(path: &Path) -> Self {
        let decoded_image = stb_image::image::load(path);
        if let stb_image::image::LoadResult::ImageU8(image) = decoded_image {
            // we are not taking into accoung pngs yet :)
            let data = (0..image.data.len() / 3)
                .map(|id| {
                    to_argb8(
                        255,
                        image.data[id * 3],
                        image.data[id * 3 + 1],
                        image.data[id * 3 + 2],
                    )
                })
                .collect();
            Self {
                width: image.width,
                height: image.height,
                data,
                depth: image.depth,
            }
        } else {
            panic!("Texture type not supported!");
        }
    }

    pub fn sample_texture(&self, uv: Vec2) -> u32 {
        let uv_coords = Vec2::new(uv.x * self.width as f32, uv.y * self.height as f32);
        let id = coords_to_index(uv_coords, self.width);

        if id < self.data.len() {
            self.data[id]
        } else {
            to_argb8(255, 255, 0, 255)
        }
    }
}
