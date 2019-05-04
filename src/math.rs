//! Various maths needed for ray tracing

use crate::Vec3;

/// Given the direction of the camera, calculate the base vectors of the image
/// plane
///
/// Given a direction, `D`, we want to find:
/// - `A` describing the vector along the X-axis of the image plane
/// - `B` describing the vector along the Y-axis of the image plane
///
/// Therefore, we have the following constraints:
/// - `A` and `D` are perpendicular: `A.dot(D) = 0`
/// - `B` and `D` are perpendicular: `B.dot(D) = 0`
/// - `A` and `B` are perpendicular: `A.dot(B) = 0`
/// - `A` must be parallel to the `Z` axis plane: `A.z = 0`
///
/// From these constraints, we can calculate `A` and `B` with one degree of
/// freedom: the magnitude of the vectors. We set this to be one.
pub fn get_camera_unit_vectors(d: &Vec3) -> (Vec3, Vec3) {
    let a = Vec3::new(1, -d.x / d.y, 0);
    let b = Vec3::new(d.x / d.y, 1, -(d.y / d.z) * 2.0);
    (a.unit(), b.unit())
}

/// Factorize `ax^2 + bx + c` given the coefficients `a`, `b`, and `c`
///
/// Returns the two possible solutions for `x`.
pub fn factorize(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let in_sqrt = b.powf(2.0) - 4.0 * a * c;
    let denom = 2.0 * a;
    if in_sqrt.is_sign_negative() || denom == 0.0 {
        return None;
    }
    Some(((-b + in_sqrt.sqrt()) / denom, (-b - in_sqrt.sqrt()) / denom))
}
