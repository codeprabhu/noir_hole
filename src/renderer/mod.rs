pub mod geometry;
pub mod ray;
pub mod scene;
pub mod shading;

pub use ray::Ray;
pub use shading::Pixel;

use crate::camera::Camera;
use scene::{DISK_INNER_RADIUS, DISK_OUTER_RADIUS, EVENT_HORIZON_RADIUS};

fn hash(mut n: u32) -> u32 {
    n ^= n >> 16;
    n = n.wrapping_mul(0x7feb352d);
    n ^= n >> 15;
    n = n.wrapping_mul(0x846ca68b);
    n ^ (n >> 16)
}

pub fn render(width: usize, height: usize, time: f32, camera: &Camera) -> Vec<Pixel> {
    let mut pixels = vec![Pixel::black(); width * height];
    let frame = camera.frame(width, height);
    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let seed = hash((x as u32).wrapping_mul(73856093) ^ (y as u32).wrapping_mul(19349663));
            let ray = frame.generate_ray_at(x as f32 + 0.5, y as f32 + 0.5);
            pixels[index] = trace_ray(ray, time, seed);
        }
    }
    pixels
}

fn trace_ray(ray: Ray, time: f32, seed: u32) -> Pixel {
    let mut position = ray.origin;
    let mut direction = ray.direction;
    let mut previous_y = position.y;

    let closest_t = -position.dot(direction);
    if closest_t <= 0.0 {
        return star_pixel(seed, time);
    }
    let closest_point = position + direction * closest_t;
    if closest_point.dot(closest_point) > 5.5 * 5.5 {
        return star_pixel(seed, time);
    }

    for _ in 0..96 {
        let radius = position.length();
        if radius <= EVENT_HORIZON_RADIUS {
            return Pixel::black();
        }
        if radius > 14.0 && position.dot(direction) > 0.0 {
            return star_pixel(seed, time);
        }

        // Numerical approximation of null-ray bending near the black hole.
        let gravity = 0.24 / radius.max(EVENT_HORIZON_RADIUS).powi(2);
        let step = if radius > 6.0 {
            0.30
        } else if radius > 3.0 {
            0.16
        } else {
            0.08
        };
        direction = (direction - position.normalized() * (gravity * step)).normalized();
        let next_position = position + direction * step;

        if (previous_y > 0.0 && next_position.y <= 0.0)
            || (previous_y < 0.0 && next_position.y >= 0.0)
        {
            let fraction = previous_y / (previous_y - next_position.y);
            let crossing = position + (next_position - position) * fraction;
            let disk_radius = (crossing.x * crossing.x + crossing.z * crossing.z).sqrt();
            if (1.5..=4.0).contains(&disk_radius) {
                return disk_pixel(disk_radius, time, seed);
            }
        }

        previous_y = next_position.y;
        position = next_position;
    }

    star_pixel(seed, time)
}

fn star_pixel(seed: u32, time: f32) -> Pixel {
    if seed % 137 != 0 {
        return Pixel::black();
    }
    let twinkle = (time + (seed % 1000) as f32 * 0.01).sin() * 0.5 + 0.5;
    let brightness = (65.0 + twinkle * 150.0) as u8;
    match seed % 5 {
        0 => Pixel::new(brightness, brightness, brightness),
        1 | 2 => Pixel::new(brightness / 2, brightness / 4, brightness),
        _ => Pixel::new(brightness, brightness / 5, brightness / 3),
    }
}

fn disk_pixel(radius: f32, time: f32, seed: u32) -> Pixel {
    let radial =
        ((DISK_OUTER_RADIUS - radius) / (DISK_OUTER_RADIUS - DISK_INNER_RADIUS)).clamp(0.0, 1.0);
    let swirl = (radius * 8.0 - time * 2.0 + seed as f32 * 0.0001)
        .sin()
        .abs();
    let glow = (radial * 0.72 + swirl * 0.28).clamp(0.0, 1.0);
    Pixel::new(
        170 + (glow * 85.0) as u8,
        (12.0 + glow * 115.0) as u8,
        (glow * 18.0) as u8,
    )
}
