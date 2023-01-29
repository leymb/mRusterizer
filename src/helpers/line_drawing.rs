use glam::{UVec2, Vec2, Vec4};

use crate::helper::{get_pixel_index, to_argb8};

#[path = "./helper.rs"]
mod helper;

fn plot_color(
    pixel: &UVec2,
    brightness: f32,
    buffer: &mut Vec<u32>,
    color: &Vec4,
    window_width: usize,
) {
    // apply brightness
    let t_color = to_argb8(
        (color.x * brightness) as u8,
        (color.y * brightness) as u8,
        (color.z * brightness) as u8,
        color.w as u8,
    );

    let t_index = get_pixel_index(&pixel, window_width);
    buffer[t_index] = t_color;
}

fn round(x: f32) -> f32 {
    (x + 0.5).floor()
}

fn fpart(x: f32) -> f32 {
    x - x.floor()
}

fn rfpart(x: f32) -> f32 {
    1.0f32 - fpart(x)
}

// based on xiaolin wu's line drawing algorithm
pub fn draw_line(
    mut point_a: Vec2,
    mut point_b: Vec2,
    buffer: &mut Vec<u32>,
    color: Vec4,
    window_width: usize,
) {
    let steep = (point_b.y - point_a.y).abs() > (point_b.x - point_a.x).abs();

    if steep {
        let t_temp_a = point_a.y;
        point_a.y = point_a.x;
        point_a.x = t_temp_a;

        std::mem::swap(&mut point_b.y, &mut point_b.x);
    }

    if point_a.x > point_b.x {
        std::mem::swap(&mut point_a.x, &mut point_b.x);
        std::mem::swap(&mut point_a.y, &mut point_b.y);
    }

    let dx = point_b.x - point_a.x;
    let dy = point_b.y - point_a.y;
    let gradient: f32;

    if dx == 0.0 {
        gradient = 1.0;
    } else {
        gradient = dy / dx
    };

    let mut xend = round(point_a.x);
    let mut yend = point_a.y + gradient * (xend - point_a.x);
    let mut xgap = rfpart(point_a.x + 0.5f32);
    let xpxl1 = xend;
    let ypxl1 = yend.floor();

    if steep {
        let pixel_a = UVec2 {
            x: (ypxl1 as u32),
            y: (xpxl1 as u32),
        };
        let pixel_b = UVec2 {
            x: (ypxl1 + 1.0f32) as u32,
            y: xpxl1 as u32,
        };
        plot_color(&pixel_a, rfpart(yend) * xgap, buffer, &color, window_width);
        plot_color(&pixel_b, fpart(yend) * xgap, buffer, &color, window_width);
    } else {
        let pixel_a = UVec2 {
            x: (xpxl1 as u32),
            y: (ypxl1 as u32),
        };
        let pixel_b = UVec2 {
            x: (xpxl1) as u32,
            y: (ypxl1 + 1.0f32) as u32,
        };
        plot_color(&pixel_a, rfpart(yend) * xgap, buffer, &color, window_width);
        plot_color(&pixel_b, fpart(yend) * xgap, buffer, &color, window_width);
    }

    let mut intery = yend + gradient;
    xend = round(point_b.x);
    yend = point_b.y + gradient * (xend - point_b.x);
    xgap = fpart(point_b.x + 0.5f32);
    let xpxl2 = xend;
    let ypxl2 = yend.floor();

    if steep {
        plot_color(
            &UVec2 {
                x: ypxl2 as u32,
                y: xpxl2 as u32,
            },
            rfpart(yend) * xgap,
            buffer,
            &color,
            window_width,
        );
        plot_color(
            &UVec2 {
                x: (ypxl2 + 1.0f32) as u32,
                y: xpxl2 as u32,
            },
            fpart(yend) * xgap,
            buffer,
            &color,
            window_width,
        );
    } else {
        plot_color(
            &UVec2 {
                x: xpxl2 as u32,
                y: ypxl2 as u32,
            },
            rfpart(yend) * xgap,
            buffer,
            &color,
            window_width,
        );
        plot_color(
            &UVec2 {
                x: xpxl2 as u32,
                y: (ypxl2 + 1.0f32) as u32,
            },
            fpart(yend) * xgap,
            buffer,
            &color,
            window_width,
        );
    }

    if steep {
        for x in (xpxl1 + 1.0f32) as u32..(xpxl2 - 1.0f32) as u32 {
            plot_color(
                &UVec2 {
                    x: intery.floor() as u32,
                    y: x,
                },
                rfpart(intery),
                buffer,
                &color,
                window_width,
            );
            plot_color(
                &UVec2 {
                    x: (intery.floor() + 1.0f32) as u32,
                    y: x,
                },
                fpart(intery),
                buffer,
                &color,
                window_width,
            );
            intery += gradient;
        }
    } else {
        for x in (xpxl1 + 1.0f32) as u32..(xpxl2 - 1.0f32) as u32 {
            plot_color(
                &UVec2 {
                    x: x,
                    y: intery.floor() as u32,
                },
                rfpart(intery),
                buffer,
                &color,
                window_width,
            );
            plot_color(
                &UVec2 {
                    x: x,
                    y: (intery.floor() + 1.0f32) as u32,
                },
                fpart(intery),
                buffer,
                &color,
                window_width,
            );
            intery += gradient;
        }
    }
}
