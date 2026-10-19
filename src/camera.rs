use crate::math::Vec3;
use crate::renderer::Ray;

pub struct Camera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub vertical_fov: f32,
}

pub struct CameraFrame {
    origin: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    aspect: f32,
    inv_width: f32,
    inv_height: f32,
    scale: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.25,
            distance: 10.0,
            vertical_fov: 60.0_f32.to_radians(),
        }
    }

    pub fn position(&self) -> Vec3 {
        let horizontal = self.distance * self.pitch.cos();
        Vec3::new(
            horizontal * self.yaw.sin(),
            self.distance * self.pitch.sin(),
            horizontal * self.yaw.cos(),
        )
    }

    #[allow(dead_code)]
    pub fn generate_ray(&self, px: usize, py: usize, width: usize, height: usize) -> Ray {
        self.frame(width, height)
            .generate_ray_at(px as f32 + 0.5, py as f32 + 0.5)
    }

    #[allow(dead_code)]
    pub fn generate_ray_at(&self, px: f32, py: f32, width: usize, height: usize) -> Ray {
        self.frame(width, height).generate_ray_at(px, py)
    }

    pub fn frame(&self, width: usize, height: usize) -> CameraFrame {
        let origin = self.position();
        let forward = (Vec3::default() - origin).normalized();
        let right = forward.cross(Vec3::new(0.0, 1.0, 0.0)).normalized();
        let up = right.cross(forward).normalized();
        CameraFrame {
            origin,
            forward,
            right,
            up,
            aspect: width as f32 / height as f32,
            inv_width: 1.0 / width as f32,
            inv_height: 1.0 / height as f32,
            scale: (self.vertical_fov * 0.5).tan(),
        }
    }

    pub fn orbit(&mut self, dx: f32, dy: f32) {
        self.yaw += dx;
        self.pitch = (self.pitch + dy).clamp(-1.45, 1.45);
    }

    pub fn zoom(&mut self, amount: f32) {
        self.distance = (self.distance + amount).clamp(4.0, 24.0);
    }
}

impl CameraFrame {
    pub fn generate_ray_at(&self, px: f32, py: f32) -> Ray {
        let screen_x = (2.0 * px * self.inv_width - 1.0) * self.aspect * self.scale;
        let screen_y = (1.0 - 2.0 * py * self.inv_height) * self.scale;
        Ray::new(
            self.origin,
            (self.forward + self.right * screen_x + self.up * screen_y).normalized(),
        )
    }
}
