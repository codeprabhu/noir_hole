use super::Ray;

#[allow(dead_code)]
pub fn hit_sphere(ray: Ray, radius: f32) -> Option<f32> {
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * ray.origin.dot(ray.direction);
    let c = ray.origin.dot(ray.origin) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }
    let root = discriminant.sqrt();
    let near = (-b - root) / (2.0 * a);
    let far = (-b + root) / (2.0 * a);
    if near > 0.001 {
        Some(near)
    } else if far > 0.001 {
        Some(far)
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn hit_disk(ray: Ray) -> Option<(f32, f32)> {
    if ray.direction.y.abs() < 1e-6 {
        return None;
    }
    let distance = -ray.origin.y / ray.direction.y;
    if distance <= 0.001 {
        return None;
    }
    let point = ray.at(distance);
    let radius = (point.x * point.x + point.z * point.z).sqrt();
    (1.5..=4.0).contains(&radius).then_some((distance, radius))
}
