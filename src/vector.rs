use std::fmt::{self, Display, Formatter};
use std::num::ParseFloatError;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign,
};
use std::str::FromStr;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vector3D([f32; 3]);

impl Vector3D {
    pub fn new(e1: f32, e2: f32, e3: f32) -> Vector3D {
        Vector3D([e1, e2, e3])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn r(&self) -> f32 {
        self.0[0]
    }

    pub fn g(&self) -> f32 {
        self.0[1]
    }

    pub fn b(&self) -> f32 {
        self.0[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(*self)
    }

    pub fn unit(&self) -> Vector3D {
        *self / self.length()
    }

    pub fn dot(&self, other: Vector3D) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }
}

impl From<[f32; 3]> for Vector3D {
    fn from(other: [f32; 3]) -> Vector3D {
        Vector3D::new(other[0], other[1], other[2])
    }
}

impl Index<usize> for Vector3D {
    type Output = f32;

    fn index(&self, ix: usize) -> &Self::Output {
        &self.0[ix]
    }
}

impl IndexMut<usize> for Vector3D {
    fn index_mut(&mut self, ix: usize) -> &mut Self::Output {
        &mut self.0[ix]
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Self::Output {
        Vector3D::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, other: Vector3D) {
        *self = *self + other;
    }
}

impl Mul for Vector3D {
    type Output = Vector3D;

    fn mul(self, other: Vector3D) -> Self::Output {
        Vector3D::new(
            self.x() * other.x(),
            self.y() * other.y(),
            self.z() * other.z(),
        )
    }
}

impl MulAssign for Vector3D {
    fn mul_assign(&mut self, other: Vector3D) {
        *self = *self * other;
    }
}

impl Mul<f32> for Vector3D {
    type Output = Vector3D;

    fn mul(self, other: f32) -> Self::Output {
        Vector3D::new(self.x() * other, self.y() * other, self.z() * other)
    }
}

impl MulAssign<f32> for Vector3D {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}

impl Div for Vector3D {
    type Output = Vector3D;

    fn div(self, other: Vector3D) -> Self::Output {
        Vector3D::new(
            self.x() / other.x(),
            self.y() / other.y(),
            self.z() / other.z(),
        )
    }
}

impl DivAssign for Vector3D {
    fn div_assign(&mut self, other: Vector3D) {
        *self = *self / other;
    }
}

impl Div<f32> for Vector3D {
    type Output = Vector3D;

    fn div(self, other: f32) -> Self::Output {
        Vector3D::new(self.x() / other, self.y() / other, self.z() / other)
    }
}

impl DivAssign<f32> for Vector3D {
    fn div_assign(&mut self, other: f32) {
        *self = *self / other;
    }
}

impl Display for Vector3D {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl FromStr for Vector3D {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words =
            s.trim().split_whitespace().map(|word| f32::from_str(word));

        let x = words.next().ok_or(ParseError::NotEnoughWords)??;
        let y = words.next().ok_or(ParseError::NotEnoughWords)??;
        let z = words.next().ok_or(ParseError::NotEnoughWords)??;

        if words.next().is_some() {
            return Err(ParseError::TooManyWords);
        }

        Ok(Vector3D::new(x, y, z))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    NotEnoughWords,
    TooManyWords,
    Parse(std::num::ParseFloatError),
}

impl From<ParseFloatError> for ParseError {
    fn from(other: ParseFloatError) -> ParseError {
        ParseError::Parse(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_vector() {
        let src = "12.3 45.6 -7.89";
        let should_be = Vector3D::new(12.3, 45.6, -7.89);

        let got = Vector3D::from_str(src).unwrap();
        assert_eq!(got, should_be);
    }
}
