use glam::Mat4;

use self::transform::Transform;

#[path = "./geometry/transform.rs"]
pub mod transform;

pub struct Camera {
    pub near_plane: f32,
    pub far_plane: f32,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub transform: Transform,
    pub movement_speed: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            near_plane: 0.1,
            far_plane: 100.0,
            fov: std::f32::consts::PI / 4.0,
            aspect_ratio: 1.0,
            transform: Transform::IDENTITY_MAT,
            movement_speed: 1.0,
        }
    }
}

impl Camera {
    pub fn get_proj_mat(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near_plane, self.far_plane)
    }

    pub fn get_view_mat(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.transform.translation,
            self.transform.translation + self.transform.forward(),
            self.transform.up(),
        )
    }
}
