use nalgebra::{Vector2};

#[inline]
pub fn get_angle_and_magnitude(p1: &Vector2<f32>, p2: &Vector2<f32>) -> (f32, f32) {
    let diff_vec = p2 - p1;
    (diff_vec.y.atan2(diff_vec.x), diff_vec.magnitude())
}

#[inline]
pub fn vec_from_angle_and_mag(angle: f32, magnitude: f32) -> Vector2<f32> {
    Vector2::new(magnitude * angle.cos(), magnitude * angle.sin())
}
