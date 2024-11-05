use cgmath::Vector3;

#[derive(Debug)]
pub struct Ray {
    orig: Vector3<f64>,
    dir: Vector3<f64>,
}

impl Ray {
    pub fn origin(&self) -> Vector3<f64> {
        return self.orig;
    }

    pub fn direction(&self) -> Vector3<f64> {
        return self.dir;
    }

    /// Returns the vector at distance `t` from the ray origin along the ray direction
    pub fn at(&self, t: f64) -> Vector3<f64> {
        return self.orig + t * self.dir;
    }

    pub fn new(orig: Vector3<f64>, dir: Vector3<f64>) -> Ray {
        Ray { orig, dir }
    }
}

/// Short [constructor](Ray::new) for a Ray
pub fn ray(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
    Ray::new(origin, direction)
}
