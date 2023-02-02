use glam::{UVec2, Vec2, Vec3};

pub fn to_argb8(a: u8, r: u8, g: u8, b: u8) -> u32 {
    let mut argb: u32 = a as u32; // alpha
    argb = (argb << 8) + r as u32; // red channel
    argb = (argb << 8) + g as u32; // green channel
    argb = (argb << 8) + b as u32; // blue channel

    argb
}

pub fn from_argb8(color: u32) -> (u8, u8, u8, u8) {
    let a = (color >> 24) as u8;
    let r = (color >> 16) as u8;
    let g = (color >> 8) as u8;
    let b = color as u8;

    (a, r, g, b)
}

// color as argb
pub fn get_pixel_index(pixel_pos: &UVec2, width: usize) -> usize {
    width * pixel_pos.y as usize + pixel_pos.x as usize
}

pub fn edge_function(v0: Vec2, v1: Vec2, p: Vec2) -> f32 {
    (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

pub fn index_to_coords(p: usize, width: usize) -> (usize, usize) {
    (p % width, p / width)
}

pub fn coords_to_index(coords: Vec2, width: usize) -> usize {
    coords.x as usize + coords.y as usize * width
}

pub fn barycentric_coords(point: Vec2, v0: Vec2, v1: Vec2, v2: Vec2, area: f32) -> Option<Vec3> {
    let m0 = edge_function(point, v1, v2);
    let m1 = edge_function(point, v2, v0);
    let m2 = edge_function(point, v0, v1);

    let a = 1.0 / area;
    if m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0 {
        Some(glam::vec3(m0 * a, m1 * a, m2 * a))
    } else {
        None
    }
}

pub fn lerp<T>(start: T, end: T, alpha: f32) -> T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Mul<f32, Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{
    start + (end - start) * alpha
}

pub fn map_to_range<T>(v: T, a1: T, a2: T, b1: T, b2: T) -> T
where
    T: std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + Copy,
{
    b1 + (v - a1) * (b2 - b1) / (a2 - a1)
}

pub fn clear_buffer<T>(buffer: &mut [T], clear_value: T)
where
    T: Copy,
{
    buffer.iter_mut().map(|x| *x = clear_value).count();
}
