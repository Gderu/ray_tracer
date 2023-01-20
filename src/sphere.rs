use crate::hittable::*;
use crate::material::*;

#[derive(Clone,)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Sphere {center, radius, mat}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut res = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vec3::zeros(),
            front_face: false,
            mat: self.mat.clone(),
        };
        res.set_face_normal(r, (res.p - self.center) / self.radius);

        Some(res)
    }
}

unsafe impl Sync for Sphere {}
unsafe impl Send for Sphere {}