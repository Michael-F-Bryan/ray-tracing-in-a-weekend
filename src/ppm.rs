use crate::{Ray, Vector3D};
use std::io::{self, Write};

pub const WHITE: Vector3D = Vector3D::new(1.0, 1.0, 1.0);
pub const BLUE: Vector3D = Vector3D::new(0.5, 0.7, 1.0);

pub fn header<W: Write>(w: &mut W, width: usize, height: usize) -> io::Result<()> {
    writeln!(w, "P3")?;
    writeln!(w, "{} {}", width, height)?;
    writeln!(w, "255")?;

    Ok(())
}

pub fn pixel<W: Write>(w: &mut W, p: Vector3D) -> io::Result<()> {
    let r = (p.r() * 256.0).clamp(0.0, 255.0) as u8;
    let g = (p.g() * 256.0).clamp(0.0, 255.0) as u8;
    let b = (p.b() * 256.0).clamp(0.0, 255.0) as u8;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bottom_is_white() {
        let dir = Vector3D::new(0.0, -2.0, 0.0);
        let ray = Ray::new(Vector3D::default(), dir);

        let got = background(ray);

        assert_eq!(got, BLUE);
        let mut buffer = Vec::new();
        pixel(&mut buffer, got).unwrap();
        assert_eq!("128 179 255\n", String::from_utf8(buffer).unwrap());
    }
}
