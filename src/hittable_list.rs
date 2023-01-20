use crate::hittable::*;

type SafeHittable = dyn Hittable + Sync + Send;
#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Arc<SafeHittable>>,
}

impl HittableList {
    pub fn zeros() -> Self {
        HittableList {objects: vec![]}
    }

    pub fn new(object: Arc<SafeHittable>) -> Self {
        HittableList {objects: vec![object]}
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<SafeHittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut res = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                res = Some(temp_rec);
            }
        }

        res
    }
}

unsafe impl Sync for HittableList {}