use cgmath::{dot, num_traits::pow, point3, vec3, InnerSpace, Point3, Vector3, Zero};

use crate::color::{color, write_color};

pub mod color;
pub mod hittable;
pub mod ray;

fn hit_sphere(center: &Vector3<f64>, radius: f64, r: &ray::Ray) -> f64 {
    let oc = center - r.origin();

    let a = r.direction().magnitude2();
    let h = dot(r.direction(), oc);
    let c = oc.magnitude2() - pow(radius, 2);
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

fn ray_color(r: &ray::Ray) -> Vector3<f64> {
    let sphere_p = vec3(0.0, 0.0, -1.0);

    let t = hit_sphere(&sphere_p, 0.5, r);

    if t > 0.0 {
        let surface = vec3(0.0, 0.0, -1.0);

        let nrm = (r.at(t) - surface).normalize();
        return 0.5 * vec3(nrm.x + 1.0, nrm.y + 1.0, nrm.z + 1.0);
    }

    let unit_dir = r.direction().normalize();
    let a = 0.5 * (unit_dir.y + 1.0);

    return (1.0 - a) * vec3(1.0, 0.0, 1.0) + a * vec3(0.1, 0.7, 0.8);
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u64 = 800;

    let image_height = ((image_width as f64) / aspect_ratio) as u64;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);

    let camera_center = vec3(0.0, 0.0, 0.0);

    // Vectors for horizontal/vertical viewport edges
    let viewport_u = vec3(viewport_width, 0.0, 0.0);
    let viewport_v = vec3(0.0, -viewport_height, 0.0);

    let pixel_du = viewport_u / image_width as f64;
    let pixel_dv = viewport_v / image_height as f64;

    // Computer upper left pixel coord
    let viewport_upper_left =
        camera_center - vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_du + pixel_dv);

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for y in 0..image_height {
        for x in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_du * x as f64) + (pixel_dv * y as f64);
            let ray_direction = pixel_center - camera_center;
            let ray = ray::Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
            write_color(pixel_color);
        }
    }
}
