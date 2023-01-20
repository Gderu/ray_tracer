use crate::hittable::*;

pub trait Material {
    fn scatter(&self, r: Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self{
        Lambertian {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        Some((self.albedo, Ray::new(rec.p, scatter_direction)))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self{
        Metal {
            albedo,
            fuzz: match fuzz < 1.0 {
                true => fuzz,
                false => 1.0,
            },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(r.direction().as_unit_vector(), rec.normal);
        match dot(reflected, rec.normal) > 0.0 {
            true => Some((self.albedo, Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere()))),
            false => None,
        }
    }
}

pub struct Dielectric {
    ir: f64, //Index of refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}


impl Material for Dielectric {
    fn scatter(&self, r: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = match rec.front_face {
            true => 1.0 / self.ir,
            false => self.ir,
        };
        let unit_direction = r.direction().as_unit_vector();
        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut rng = random_fast_rng::FastRng::new();

        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) >  rng.gen() {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(rec.p, direction)))
    }
}