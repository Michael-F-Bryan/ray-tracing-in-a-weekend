use ray_tracing_in_a_weekend::{ppm, Hitable, Ray, Sphere, Vector3D};
use std::io::{self, Write};

pub fn main() -> io::Result<()> {
    let width = 1000;
    let height = 500;
    let mut out = io::stdout();

    let lower_left = Vector3D::new(-2.0, -1.0, 1.0);
    let horizontal = Vector3D::new(4.0, 0.0, 0.0);
    let vertical = Vector3D::new(0.0, 2.0, 0.0);
    let origin = Vector3D::default();
    let sphere = Sphere::new(Vector3D::new(0.0, 0.0, 1.0), 0.5);

    ppm::header(&mut out, width, height)?;
    writeln!(out)?;

    for y in 0..height {
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
    if let Some(hit) = sphere.hit(ray, 0.0, std::f32::INFINITY) {
        0.5 * (hit.normal + Vector3D::new(1.0, 1.0, 1.0))
    } else {
        ppm::background(ray)
    }
}
