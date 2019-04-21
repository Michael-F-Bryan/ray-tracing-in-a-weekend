use crate::Vector3D;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Ray {
    pub start: Vector3D,
    pub direction: Vector3D,
}

impl Ray {
    pub fn new(start: Vector3D, direction: Vector3D) -> Ray {
        Ray { start, direction }
    }

    pub fn point_at(&self, t: f32) -> Vector3D {
        self.start + t * self.direction
    }
}
