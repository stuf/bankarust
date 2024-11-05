use cgmath::{Point3, Vector3};

use crate::ray::Ray;

#[derive(Debug)]
pub struct Hittable {
    p: Point3<f64>,
    normal: Vector3<f64>,
    t: f64,
}

impl Hittable {
    pub fn hit(r: &Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &Hittable) -> bool {
        return false;
    }
}
