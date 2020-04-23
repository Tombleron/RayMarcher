mod linear;
mod primitives;
mod scene;

use scene::{Scene, Camera};
use image::{RgbImage, Pixel};
use linear::Vector;
use primitives::Sphere;


fn main() {

    let frames: u32 = 1;
    let size: u32 = 200;

    for x in 0..frames {
        let sphere: Sphere = Sphere::new(10, Vector::new(0.0,0.0,20.0), 1);
        let sphere2: Sphere = Sphere::new(5, Vector::new(10.0, 0.0,20.0), 2);
       
        let camera: Camera = Camera::new([size, size], 60.0, Vector::new(0.0,0.0,0.0));

        let mut img: RgbImage = RgbImage::new(size, size);
        let time: f64 = 2.0 * std::f64::consts::PI * (x as f64/ frames as f64); 
        
        let scene: Scene = Scene::new(vec![sphere, sphere2], vec![Vector::new(0.0, -10.0 + (20.0 * (x as f64/ frames as f64)), 0.0)], time, camera);
        
        scene.render(&mut img);
        img.save(format!("frames/{}.png", x)).unwrap();
        println!("{}", x);
    }
}
