extern crate minifb;

#[path = "./helpers/helper.rs"]
mod helper;

#[path = "./helpers/line_drawing.rs"]
mod line_drawing;

use glam::{UVec2, Vec2, Vec4};
use helper::{edge_function, get_pixel_index, index_to_coords, to_argb8};
use line_drawing::draw_line;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let color = to_argb8(255, 0, 0, 255);
    let line_color = Vec4::new(255.0f32, 0.0f32, 255.0f32, 0.0f32);
    let center_pixel = UVec2 {
        x: (WIDTH / 2) as u32,
        y: (HEIGHT / 2) as u32,
    };

    let vertices: [Vec2; 3] = [
        glam::vec2(100.0, 100.0),
        glam::vec2(250.0, 400.0),
        glam::vec2(400.0, 100.0),
    ];

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
        for i in 0..buffer.len() {
            let coords = index_to_coords(i, WIDTH);

            //shadowing variable
            let coord = glam::vec2(coords.0 as f32, coords.1 as f32);

            // calculating determinants for each side of the triangle
            let m0 = edge_function(coord, vertices[0], vertices[1]);
            let m1 = edge_function(coord, vertices[1], vertices[2]);
            let m2 = edge_function(coord, vertices[2], vertices[0]);

            //let side = edge_function_cw(coord, edge.0, edge.1);
            if m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0 {
                buffer[i] = to_argb8(0, 255, 0, 0)
            } else {
                buffer[i] = to_argb8(255, 0, 0, 0);
            }

            buffer[get_pixel_index(center_pixel, WIDTH)] = color;
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
            )
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
