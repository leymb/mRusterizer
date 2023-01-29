extern crate minifb;

#[path = "./helpers/helper.rs"]
mod helper;

#[path = "./helpers/line_drawing.rs"]
mod line_drawing;

#[path = "./geometry/triangle.rs"]
mod triangle;

pub mod texture;

use glam::{Vec2, Vec3, Vec4};
use line_drawing::draw_line;
use minifb::{Key, Window, WindowOptions};
use std::path::Path;
use texture::Texture;
use triangle::vertex::Vertx;
use triangle::Tri;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut z_buffer = vec![f32::INFINITY; WIDTH * HEIGHT];

    let line_color = Vec4::new(255.0f32, 0.0f32, 255.0f32, 0.0f32);

    let test_tex = Texture::load(Path::new("assets/mel.jpg"));

    let test_tri = Tri {
        vert_a: Vertx {
            pos: Vec3 {
                x: 100.0,
                y: 100.0,
                z: 0.0,
            },
            color: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
            uv: glam::vec2(0.0, 0.0),
        },
        vert_b: Vertx {
            pos: Vec3 {
                x: 100.0,
                y: 400.0,
                z: 1.0,
            },
            color: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
            uv: glam::vec2(0.0, 1.0),
        },
        vert_c: Vertx {
            pos: Vec3 {
                x: 400.0,
                y: 400.0,
                z: 1.0,
            },
            color: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
            uv: glam::vec2(1.0, 1.0),
        },
    };

    let second_tri = Tri {
        vert_a: Vertx {
            pos: Vec3 {
                x: 100.0,
                y: 100.0,
                z: 0.0,
            },
            color: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            uv: glam::vec2(0.0, 0.0),
        },
        vert_b: Vertx {
            pos: Vec3 {
                x: 400.0,
                y: 400.0,
                z: 0.0,
            },
            color: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            uv: glam::vec2(1.0, 1.0),
        },
        vert_c: Vertx {
            pos: Vec3 {
                x: 400.0,
                y: 100.0,
                z: 0.0,
            },
            color: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            uv: glam::vec2(1.0, 0.0),
        },
    };

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        test_tri.raster_textured(&mut buffer, &mut z_buffer, &test_tex, WIDTH);
        second_tri.raster_textured(&mut buffer, &mut z_buffer, &test_tex, WIDTH);

        // test draw for get_pixel_index function
        //buffer[get_pixel_index(&center_pixel, WIDTH)] = color;

        draw_line(
            Vec2 {
                x: 100.0f32,
                y: 100.0f32,
            },
            Vec2 {
                x: 400.0f32,
                y: 350.0f32,
            },
            &mut buffer,
            line_color,
            WIDTH,
        );

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
