mod linear;
mod primitives;
mod scene;

use image::RgbImage;
use linear::Vector;
use primitives::Sphere;
use scene::{Camera, Scene};

fn main() {
    let frames: u32 = 60;
    let size: u32 = 1024;

    for x in 0..frames {
        let sphere: Sphere = Sphere::new(10, Vector::new(0.0, 0.0, 20.0), 1);
        let sphere2: Sphere = Sphere::new(5, Vector::new(10.0, 0.0, 20.0), 2);
        let sphere3: Sphere = Sphere::new(30, Vector::new(-20.0, 0.0, 50.0), 3);

        let camera: Camera = Camera::new([size, size], Vector::new(0.0, 0.0, 0.0));

        let mut img: RgbImage = RgbImage::new(size, size);

        let scene: Scene = Scene::new(
            vec![sphere, sphere2, sphere3],
            vec![Vector::new(
                0.0,
                -25.0 + (100.0 * ((x as f64 - frames as f64 / 2.0) / frames as f64 / 2.0)).abs(),
                0.0,
            )],
            camera,
        );

        scene.render(&mut img);
        img.save(format!("frames/{}.png", x)).unwrap();
        println!("{}", x);
    }
}
