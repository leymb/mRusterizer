extern crate minifb;

#[path = "./helpers/helper.rs"]
mod helper;

#[path = "./helpers/line_drawing.rs"]
mod line_drawing;

#[path = "./geometry/triangle.rs"]
mod triangle;

#[path = "./geometry/mesh.rs"]
mod mesh;

#[path = "./geometry/quad.rs"]
mod quad;

mod camera;

pub mod texture;

use camera::transform::Transform;
use camera::Camera;
use glam::{Vec2, Vec4};
use helper::clear_buffer;
use line_drawing::draw_line;
use minifb::{Key, Window, WindowOptions};
use quad::Quad;
use std::path::Path;
use texture::Texture;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut z_buffer = vec![f32::INFINITY; WIDTH * HEIGHT];

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let camera = Camera {
        aspect_ratio,
        transform: Transform::from_translation(glam::vec3(0.0, 0.0, 8.0)),
        far_plane: 1000.0,
        ..Default::default()
    };

    let line_color = Vec4::new(255.0f32, 0.0f32, 255.0f32, 0.0f32);

    let test_tex = Texture::load(Path::new("assets/mel.jpg"));

    let test_quad = Quad::default();

    let mut mesh_rot = std::f32::consts::FRAC_PI_4;

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
        clear_buffer(&mut buffer, 0);
        clear_buffer(&mut z_buffer, f32::INFINITY);

        let mesh_transform = Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            mesh_rot,
            0.0,
            0.0,
        ));

        // let mvp = camera.get_proj_mat() * camera.get_view_mat() * mesh_transform.get_trs_mat();

        // test_quad.raster(
        //     Some(&test_tex),
        //     &mut buffer,
        //     &mut z_buffer,
        //     Vec2::new(WIDTH as f32, HEIGHT as f32),
        //     &mvp,
        // );

        test_quad.raster_as_cube(
            Some(&test_tex),
            &mut buffer,
            &mut z_buffer,
            Vec2::new(WIDTH as f32, HEIGHT as f32),
            &mesh_transform.get_trs_mat(),
            &camera.get_view_mat(),
            &camera.get_proj_mat(),
        );

        draw_line(
            Vec2 {
                x: 10.0f32,
                y: 10.0f32,
            },
            Vec2 {
                x: 400.0f32,
                y: 350.0f32,
            },
            &mut buffer,
            line_color,
            WIDTH,
        );

        // rotate mesh
        mesh_rot += 0.05;

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
