use ray_tracing_in_a_weekend::ppm;
use ray_tracing_in_a_weekend::{Ray, Vector3D};
use std::io::{self, Write};

pub fn main() -> io::Result<()> {
    let width = 200;
    let height = 100;
    let mut out = io::stdout();

    let lower_left = Vector3D::new(-2.0, -1.0, 1.0);
    let horizontal = Vector3D::new(4.0, 0.0, 0.0);
    let vertical = Vector3D::new(0.0, 2.0, 0.0);
    let origin = Vector3D::default();
    let sphere = Sphere::new(Vector3D::new(0.0, 0.0, -1.0), 0.5);

    ppm::header(&mut out, width, height)?;
    writeln!(out)?;

    for y in (0..height).rev() {
        for x in 0..width {
            let h = (x as f32) / (width as f32);
            let v = (y as f32) / (height as f32);

            let ray = Ray::new(origin, lower_left + h * horizontal + v * vertical);
            let pixel_value = colour(sphere, ray);
            ppm::pixel(&mut out, pixel_value)?;
        }

        writeln!(out)?;
    }

    Ok(())
}

fn colour(sphere: Sphere, ray: Ray) -> Vector3D {
    if sphere.hit_by(ray) {
        Vector3D::new(1.0, 0.0, 0.0)
    } else {
        ppm::background(ray)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Sphere {
    centre: Vector3D,
    radius: f32,
}

impl Sphere {
    pub fn new(centre: Vector3D, radius: f32) -> Sphere {
        Sphere { centre, radius }
    }

    pub fn hit_by(&self, ray: Ray) -> bool {
        let oc = ray.start - self.centre;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let determinant = b * b - 4.0 * a * c;
        determinant > 0.0
    }
}
