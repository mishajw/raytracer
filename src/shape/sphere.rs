use crate::math;
use crate::shape::Shape;
use crate::shape::SurfaceNormal;
use crate::Ray;
use crate::Vec3;

/// Regular sphere shape
pub struct Sphere {
    centre: Vec3,
    radius: f64,
}

impl Sphere {
    #[allow(missing_docs)]
    pub fn new(centre: Vec3, radius: f64) -> Sphere {
        Sphere { centre, radius }
    }
}

impl Shape for Sphere {
    fn get_collision(&self, ray: &Ray) -> Option<f64> {
        // Make the ray's centre the origin to simplify
        let c = self.centre - ray.position;
        let d = ray.direction;
        let a = d.x.powf(2.0) + d.y.powf(2.0) + d.z.powf(2.0);
        let b = (2.0 * d.x * c.x) + (2.0 * d.y * c.y) + (2.0 * d.z * c.z);
        let c = c.x.powf(2.0) + c.y.powf(2.0) + c.z.powf(2.0);
        match math::factorize(a, b, c - self.radius.powf(2.0)) {
            Some((intersection_1, intersection_2)) => {
                math::closest_intersection(&[intersection_1, intersection_2])
            }
            None => None,
        }
    }
}

impl SurfaceNormal for Sphere {
    fn get_normal(&self, position: Vec3) -> Vec3 {
        (position - self.centre).unit()
    }
}
