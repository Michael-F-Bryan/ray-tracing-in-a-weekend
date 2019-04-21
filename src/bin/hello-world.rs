use ray_tracing_in_a_weekend::{Ray, Vector3D};
use std::io::{self, Write};

pub const WHITE: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
pub const BLUE: Vector3D = Vector3D::new(0.5, 0.7, 1.0);

pub fn header<W: Write>(
    w: &mut W,
    width: usize,
    height: usize,
) -> io::Result<()> {
    writeln!(w, "P3")?;
    writeln!(w, "{} {}", width, height)?;
    writeln!(w, "255")?;

    Ok(())
}

pub fn pixel<W: Write>(w: &mut W, p: Vector3D) -> io::Result<()> {
    let r = (p.r() * 256.0) as u8;
    let g = (p.g() * 256.0) as u8;
    let b = (p.b() * 256.0) as u8;

    writeln!(w, "{} {} {}", r, g, b)
}

pub fn background(ray: Ray) -> Vector3D {
    let direction = ray.direction.unit();
    let t = 0.5 * (1.0 + direction.y());

    lerp(BLUE, WHITE, t)
}

fn lerp(start: Vector3D, end: Vector3D, t: f32) -> Vector3D {
    debug_assert!(0.0 <= t && t <= 1.0);
    (1.0 - t) * start + t * end
}

pub fn main() -> io::Result<()> {
    let width = 200;
    let height = 100;
    let mut out = io::stdout();

    let lower_left = Vector3D::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3D::new(4.0, 0.0, 0.0);
    let vertical = Vector3D::new(0.0, 2.0, 0.0);
    let origin = Vector3D::default();

    header(&mut out, width, height)?;
    writeln!(out)?;

    for y in (0..height).rev() {
        for x in 0..width {
            let h = (x as f32) / (width as f32);
            let v = (y as f32) / (height as f32);

            let ray =
                Ray::new(origin, lower_left + h * horizontal + v * vertical);
            let colour = background(ray);

            pixel(&mut out, colour)?;
        }

        writeln!(out)?;
    }

    Ok(())
}
