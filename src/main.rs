mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod material;

use sphere::*;
use hittable::*;
use hittable_list::*;
use camera::*;
use material::*;

use image::{RgbImage, Rgb};
use std::rc::Rc;
use std::time::{Instant, Duration};
use random_fast_rng::Random;
use std::f64::consts::PI;
use rayon::prelude::*;
use std::ops::Range;

#[derive(Clone, Copy)]
struct ImageData {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
}

fn main() {
    let start_time = Instant::now();
    //Image
    let img_data = ImageData {
        aspect_ratio: 3.0 / 2.0 ,
        image_width: 1200,
        image_height: 800,
        samples_per_pixel: 500,
        max_depth: 50,
    };

    //World
    let mut world = random_scene();

    //Camera
    let look_from = Point3::new(12.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(look_from, look_at, vup, 20.0, img_data.aspect_ratio, aperture, dist_to_focus);

    //Render
    let step = 1;
    let thread_res = (0..img_data.image_height).collect::<Vec<_>>().into_par_iter().map(|y| parallel_create_picture(
        0..img_data.image_width, y..(y+step), &world, cam, img_data,
    )).collect::<Vec<_>>();

    let mut img = RgbImage::new(img_data.image_width, img_data.image_height);
    let mut y = 0;
    let mut x = 0;
    for sub_image in thread_res {
        for row in sub_image {
            for pxl in row {
                img.put_pixel(x, y, pxl);
                x += 1;
            }
            y += 1;
            x = 0;
        }
    }
    loop {
        match img.save("image.png") {
            Err(e) => println!("{:?}", e),
            _ => break,
        };
    }
    println!("Done");
    println!("End time: {:?}", start_time.elapsed());
}

fn ray_color(r: Ray, world: Box<&dyn Hittable>, depth: u32) -> Color {
    if depth <= 0 {
        return Color::zeros();
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(scattered, world, depth - 1)
        } else {
            Color::zeros() // black
        }
    } else {
        let unit_direction = r.direction().as_unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::zeros();
    let mut rng = random_fast_rng::FastRng::new();
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));
    for a in (-11)..11 {
        for b in (-11)..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(a as f64 + 0.9 * rng.gen::<f64>(), 0.2, b as f64 + 0.9 * rng.gen::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random_between(0.5, 1.0);
                    let fuzz = Color::random_between(0.0, 0.5).x();
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    //glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}

fn write_color(img: &mut RgbImage, pixel_color: Color, x: u32, y: u32, samples_per_pixel: u32) {
    let scale = 1.0 / (samples_per_pixel as f64);

    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();

    img.put_pixel(x, y, Rgb([(r * 255.999) as u8, (g * 255.999) as u8, (b * 255.999) as u8]));
}

fn parallel_create_picture(x_range: Range<u32>, y_range: Range<u32>, world: &HittableList, cam: Camera, img_data: ImageData) -> Vec<Vec<Rgb<u8>>> {
    let mut img = vec![vec![Rgb([0; 3]); x_range.len()]; y_range.len()];
    let mut rng = random_fast_rng::FastRng::new();
    let y_init = y_range.clone().into_iter().next().unwrap();
    let x_init = x_range.clone().into_iter().next().unwrap();
    let scale = 1.0 / (img_data.samples_per_pixel as f64);
    for y in y_range {
        println!("Rows completed: {}", y);
        for x in x_range.clone() {
            let i = x as f64;
            let j = (img_data.image_height - 1 - y) as f64;
            let image_width_f = (img_data.image_width - 1) as f64;
            let image_height_f = (img_data.image_height - 1) as f64;
            let mut pixel_color = Color::zeros();
            for _ in 0..img_data.samples_per_pixel {
                let u = (i + rng.gen::<f64>()) / image_width_f;
                let v = (j - rng.gen::<f64>()) / image_height_f;
                let r = cam.get_ray(u, v);
                let start_ray = Instant::now();
                pixel_color += ray_color(r, Box::new(world), img_data.max_depth);
            }

            let r = (pixel_color.x() * scale).sqrt();
            let g = (pixel_color.y() * scale).sqrt();
            let b = (pixel_color.z() * scale).sqrt();

            img[(y - y_init) as usize][(x - x_init) as usize] = Rgb([(r * 255.999) as u8, (g * 255.999) as u8, (b * 255.999) as u8]);
        }
    }
    img
}