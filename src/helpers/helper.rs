use glam::{UVec2, Vec2};

pub fn to_argb8(a: u8, r: u8, g: u8, b: u8) -> u32 {
    let mut argb: u32 = a as u32; // alpha
    argb = (argb << 8) + r as u32; // red channel
    argb = (argb << 8) + g as u32; // green channel
    argb = (argb << 8) + b as u32; // blue channel

    argb
}

// color as argb
pub fn get_pixel_index(pixel_pos: UVec2, width: usize) -> usize {
    width * pixel_pos.y as usize + pixel_pos.x as usize
}

pub fn edge_function(v0: Vec2, v1: Vec2, p: Vec2) -> f32 {
    (p.x - v0.x) * (v1.y - v0.y) - (p.y - v0.y) * (v1.x - v0.x)
}

pub fn index_to_coords(p: usize, width: usize) -> (usize, usize) {
    (p % width, p / width)
}
