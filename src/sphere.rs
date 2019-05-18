use crate::{Hit, Hitable, Ray, Vector3D};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pub centre: Vector3D,
    pub radius: f32,
}

impl Sphere {
    pub fn new(centre: Vector3D, radius: f32) -> Sphere {
        Sphere { centre, radius }
    }

    fn hit_at(&self, ray: Ray, t: f32) -> Hit {
        let hit_point = ray.point_at(t);
        let normal = (hit_point - self.centre).unit();

        Hit {
            t,
            hit_point,
            normal,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.start - self.centre;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let descriminant = b * b - a * c;

        if descriminant < 0.0 {
            return None;
        }

        let t = (-b - descriminant.sqrt()) / a;

        if t_min < t && t < t_max {
            return Some(self.hit_at(ray, t));
        }

        let t = (-b + descriminant.sqrt()) / a;

        if t_min < t && t < t_max {
            return Some(self.hit_at(ray, t));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_directly_at_the_ball() {
        let ray = Ray::new(Vector3D::ZERO, Vector3D::Z_AXIS);
        let sphere = Sphere::new(Vector3D::new(0.0, 0.0, 100.0), 1.0);

        let hit = sphere.hit(ray, 0.0, std::f32::INFINITY).unwrap();

        assert_eq!(hit.hit_point, sphere.centre - sphere.radius * ray.direction);
        assert_eq!(hit.normal, -Vector3D::Z_AXIS);
        assert_eq!(hit.t, sphere.centre.z() - sphere.radius);
    }
}
