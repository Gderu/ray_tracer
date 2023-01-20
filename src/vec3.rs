use std::ops;
use image::Rgb;
pub use random_fast_rng::Random;

pub type Point3 = Vec3; //3D point
pub type Color = Vec3; //RGB color

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    v: [f64; 3],
}

impl Vec3 {
    pub fn zeros() -> Self {
        Vec3 {
            v: [0., 0., 0.],
        }
    }

    pub fn new(v0: f64, v1: f64, v2: f64) -> Self {
        Vec3 {
            v: [v0, v1, v2],
        }
    }

    pub fn random() -> Self {
        let mut rng = random_fast_rng::FastRng::new();
        Vec3 {
            v: [rng.gen(), rng.gen(), rng.gen()]
        }
    }

    pub fn random_between(min: f64, max: f64) -> Self {
        let convert = |r: f64| (r * (max - min) + min);
        let mut rng = random_fast_rng::FastRng::new();
        Vec3 {
            v: [convert(rng.gen()), convert(rng.gen()), convert(rng.gen())]
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let res = Vec3::random();
            if res.length_squared() <= 1.0 {
                return res;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = random_fast_rng::FastRng::new();
        loop {
            let res = Vec3::new(rng.gen(), rng.gen(), 0.0);
            if res.length_squared() <= 1.0 {
                return res;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Vec3::random_in_unit_sphere().as_unit_vector()
    }

    pub fn x(&self) -> f64 {
        self.v[0]
    }

    pub fn y(&self) -> f64 {
        self.v[1]
    }

    pub fn z(&self) -> f64 {
        self.v[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2]
    }

    pub fn as_unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        self.v[0].abs() < S && self.v[1].abs() < S && self.v[2].abs() < S
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.v[0], self.v[1], self.v[2])
    }
}

impl Into<Rgb<u8>> for Vec3 {
    fn into(self) -> Rgb<u8> {
        Rgb([(self.v[0] * 255.999) as u8, (self.v[1] * 255.999) as u8, (self.v[2] * 255.999) as u8])
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.v[0] + rhs.v[0], self.v[1] + rhs.v[1], self.v[2] + rhs.v[2])
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.v[0] += rhs.v[0];
        self.v[1] += rhs.v[1];
        self.v[2] += rhs.v[2];
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.v[0], -self.v[1], -self.v[2])
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.v[0] - rhs.v[0], self.v[1] - rhs.v[1], self.v[2] - rhs.v[2])
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.v[0] * rhs.v[0], self.v[1] * rhs.v[1], self.v[2] * rhs.v[2])
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.v[0] * rhs, self.v[1] * rhs, self.v[2] * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(rhs.v[0] * self, rhs.v[1] * self, rhs.v[2] * self)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.v[0] *= rhs;
        self.v[1] *= rhs;
        self.v[2] *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        self * (1.0 / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
    v1.v[0] * v2.v[0] + v1.v[1] * v2.v[1] + v1.v[2] * v2.v[2]
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3::new(
        v1.v[1] * v2.v[2] - v1.v[2] * v2.v[1],
        v1.v[2] * v2.v[0] - v1.v[0] * v2.v[2],
        v1.v[0] * v2.v[1] - v1.v[1] * v2.v[0],
    )
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    if dot(-uv, n) > 1.0 {
        println!("{}, {}", uv, n);
    }
    let cos_theta = dot(-uv, n).min(1.0);
    let r_perp = etai_over_etat * (uv + cos_theta * n);
    let r_parallel = -(1.0 - r_perp.length_squared()).abs().sqrt() * n;
    r_perp + r_parallel
}