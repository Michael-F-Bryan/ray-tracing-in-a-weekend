use crate::{Ray, Vector3D};

pub struct Hit {
    pub t: f32,
    pub hit_point: Vector3D,
    pub normal: Vector3D,
}

pub trait Hitable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
