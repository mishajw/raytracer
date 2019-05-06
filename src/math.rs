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
    // TODO: Using `b` instead of `-b` fixes the output of this equation, but
    // I'm too tired to figure out why
    Some(((b + in_sqrt.sqrt()) / denom, (b - in_sqrt.sqrt()) / denom))
}

/// Get the angle between two vectors `a` and `b`
///
/// Result is in radians.
pub fn angle(a: Vec3, b: Vec3) -> f64 {
    (a.dot(&b) / (a.magnitude() * b.magnitude())).acos()
}

/// Get the reflection vector
///
/// Given the direction of a ray `d`, and the normal of the surface it reflects
/// off `n`, calculate the direction of the reflected ray
pub fn reflect(d: Vec3, n: Vec3) -> Vec3 { d - n * (2.0 * (d.dot(&n))) }

/// Get the closest intersection point on a ray
///
/// Given a list of locations on a ray (where 0 is at the start of the ray, and
/// >0 is along the ray), get the closest value to the start of the ray.
pub fn closest_intersection(intersections: &[f64]) -> Option<f64> {
    intersections
        .into_iter()
        // Negative means that its before the ray
        .filter(|a| **a >= 0.0)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .map(|a| *a)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_factorize() {
        let (a, b) = factorize(12.0, 20.0, 3.0).unwrap();
        assert!((a - 3.0 / 2.0).abs() < 1e-6);
        assert!((b - 1.0 / 6.0).abs() < 1e-6);
    }
}
