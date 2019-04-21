use std::io::{self, Write};

fn header<W: Write>(w: &mut W, width: usize, height: usize) -> io::Result<()> {
    writeln!(w, "P3")?;
    writeln!(w, "{} {}", width, height)?;
    writeln!(w, "255")?;

    Ok(())
}

fn pixel<W: Write>(w: &mut W, r: f32, g: f32, b: f32) -> io::Result<()> {
    let r = (r * 256.0) as u8;
    let g = (g * 256.0) as u8;
    let b = (b * 256.0) as u8;

    write!(w, "{} {} {}", r, g, b)
}

fn main() -> io::Result<()> {
    let width = 200;
    let height = 100;
    let mut out = io::stdout();

    header(&mut out, width, height)?;

    for y in (0..height).rev() {
        for x in 0..width {
            let r = (x as f32)/(width as f32);
            let g = (y as f32)/(height as f32);
            let b = 0.2;

            if x > 0 {
                write!(&mut out, "\t")?;
            }

            pixel(&mut out, r, g, b)?;
        }

        write!(&mut out, "\n")?;
    }

    Ok(())
}