use ray_tracing_in_a_weekend::{Ray, Vector3D};
use std::io::{self, Write};
use ray_tracing_in_a_weekend::ppm;

pub fn main() -> io::Result<()> {
    let width = 200;
    let height = 100;
    let mut out = io::stdout();

    let lower_left = Vector3D::new(-2.0, -1.0, 1.0);
    let horizontal = Vector3D::new(4.0, 0.0, 0.0);
    let vertical = Vector3D::new(0.0, 2.0, 0.0);
    let origin = Vector3D::default();

    ppm::header(&mut out, width, height)?;
    writeln!(out)?;

    for y in (0..height).rev() {
        for x in 0..width {
            let h = (x as f32) / (width as f32);
            let v = (y as f32) / (height as f32);

            let ray =
                Ray::new(origin, lower_left + h * horizontal + v * vertical);
            let colour = ppm::background(ray);

            ppm::pixel(&mut out, colour)?;
        }

        writeln!(out)?;
    }

    Ok(())
}
