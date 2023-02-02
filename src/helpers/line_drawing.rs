use glam::{UVec2, Vec2, Vec4};

use crate::helper::{from_argb8, get_pixel_index, lerp, to_argb8};

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
        (color.w * brightness) as u8,
    );

    let pixel_center = get_pixel_index(&pixel, window_width);

    let pixel_up = get_pixel_index(
        &UVec2 {
            x: pixel.x,
            y: pixel.y - 1,
        },
        window_width,
    );
    let pixel_down = get_pixel_index(
        &UVec2 {
            x: pixel.x,
            y: pixel.y + 1,
        },
        window_width,
    );
    let pixel_left = get_pixel_index(
        &UVec2 {
            x: pixel.x - 1,
            y: pixel.y,
        },
        window_width,
    );
    let pixel_right = get_pixel_index(
        &UVec2 {
            x: pixel.x + 1,
            y: pixel.y,
        },
        window_width,
    );

    if pixel_center < buffer.len()
        && pixel_down < buffer.len()
        && pixel_up < buffer.len()
        && pixel_left < buffer.len()
        && pixel_right < buffer.len()
    {
        let pixel_up_color = from_argb8(buffer[pixel_up]);
        let pixel_down_color = from_argb8(buffer[pixel_down]);
        let pixel_left_color = from_argb8(buffer[pixel_left]);
        let pixel_right_color = from_argb8(buffer[pixel_right]);

        let inverse = 1.0 / 4.0;

        let a_average = ((pixel_up_color.0 as f32
            + pixel_down_color.0 as f32
            + pixel_left_color.0 as f32
            + pixel_right_color.0 as f32)
            * inverse)
            * brightness;

        let r_average = ((pixel_up_color.1 as f32
            + pixel_down_color.1 as f32
            + pixel_left_color.1 as f32
            + pixel_right_color.1 as f32)
            * inverse)
            * brightness;

        let g_average = ((pixel_up_color.2 as f32
            + pixel_down_color.2 as f32
            + pixel_left_color.2 as f32
            + pixel_right_color.2 as f32)
            * inverse)
            * brightness;

        let b_average = ((pixel_up_color.3 as f32
            + pixel_down_color.3 as f32
            + pixel_left_color.3 as f32
            + pixel_right_color.3 as f32)
            * inverse)
            * brightness;

        buffer[pixel_center] = to_argb8(
            lerp(color.x, a_average, 1.0 - brightness) as u8,
            lerp(color.y, r_average, 1.0 - brightness) as u8,
            lerp(color.z, g_average, 1.0 - brightness) as u8,
            lerp(color.w, b_average, 1.0 - brightness) as u8,
        );
    }
    //buffer[pixel_center] = t_color;
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
