use glam::{Mat4, Vec2, Vec3};

use crate::{
    camera::transform::{self, Transform},
    mesh::Mesh,
    texture::Texture,
    triangle::{vertex::Vertx, Tri},
};

pub struct Quad {
    mesh: Mesh,
}

impl Quad {
    pub fn new(vertices: [Vertx; 4]) -> Self {
        let triangle1 = Tri::new([vertices[0], vertices[1], vertices[2]]);
        let triangle2 = Tri::new([vertices[0], vertices[2], vertices[3]]);

        let mut mesh = Mesh::default();
        mesh.add_section(&mut [triangle1, triangle2]);

        Self { mesh }
    }

    pub fn raster(
        &self,
        texture: Option<&Texture>,
        buffer: &mut [u32],
        z_buffer: &mut [f32],
        window_size: Vec2,
        mvp_mat: &Mat4,
    ) {
        self.mesh
            .raster(texture, buffer, z_buffer, window_size, mvp_mat);
    }

    pub fn raster_as_cube(
        &self,
        texture: Option<&Texture>,
        buffer: &mut [u32],
        z_buffer: &mut [f32],
        window_size: Vec2,
        parent_model: &Mat4,
        view: &Mat4,
        projection: &Mat4,
    ) {
        let proj_view = *projection * *view;

        // z+
        let front_transform = Transform::IDENTITY_MAT;

        // z-
        let back_transform = Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            -std::f32::consts::PI,
            0.0,
            0.0,
        ));

        // y+
        let up_transform = Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            std::f32::consts::FRAC_PI_2,
            0.0,
            0.0,
        ));

        // y-
        let down_transform = Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_2,
            0.0,
            0.0,
        ));

        // x+
        let right_transform = Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            0.0,
            -std::f32::consts::FRAC_PI_2,
            0.0,
        ));

        // x-
        let left_transform = Transform::from_rotation(glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            0.0,
            std::f32::consts::FRAC_PI_2,
            0.0,
        ));

        self.raster(
            texture,
            buffer,
            z_buffer,
            window_size,
            &(proj_view * (*parent_model * front_transform.get_trs_mat())),
        );
        self.raster(
            texture,
            buffer,
            z_buffer,
            window_size,
            &(proj_view * (*parent_model * back_transform.get_trs_mat())),
        );
        self.raster(
            texture,
            buffer,
            z_buffer,
            window_size,
            &(proj_view * (*parent_model * left_transform.get_trs_mat())),
        );
        self.raster(
            texture,
            buffer,
            z_buffer,
            window_size,
            &(proj_view * (*parent_model * right_transform.get_trs_mat())),
        );
        self.raster(
            texture,
            buffer,
            z_buffer,
            window_size,
            &(proj_view * (*parent_model * up_transform.get_trs_mat())),
        );
        self.raster(
            texture,
            buffer,
            z_buffer,
            window_size,
            &(proj_view * (*parent_model * down_transform.get_trs_mat())),
        );
    }
}

impl Default for Quad {
    fn default() -> Self {
        let vert_a = Vertx {
            pos: Vec3 {
                x: -1.0,
                y: -1.0,
                z: 1.0,
            },
            color: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
            uv: glam::vec2(0.0, 0.0),
        };

        let vert_b = Vertx {
            pos: Vec3 {
                x: -1.0,
                y: 1.0,
                z: 1.0,
            },
            color: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
            uv: glam::vec2(0.0, 1.0),
        };

        let vert_c = Vertx {
            pos: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            color: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
            uv: glam::vec2(1.0, 1.0),
        };

        let vert_d = Vertx {
            pos: Vec3 {
                x: 1.0,
                y: -1.0,
                z: 1.0,
            },
            color: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            uv: glam::vec2(1.0, 0.0),
        };

        Self::new([vert_a, vert_b, vert_c, vert_d])
    }
}
