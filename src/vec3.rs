/// Vector with 3 dimensions
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    #[allow(missing_docs)]
    pub fn new<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(
        x: X,
        y: Y,
        z: Z,
    ) -> Self
    {
        Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}
