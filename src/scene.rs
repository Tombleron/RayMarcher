//! This module contains scene interactions.
use super::linear::Vector;
use super::primitives::{Ray, Sphere};
use image::RgbImage;

/// Camera object.
///
/// Curently don't have direction.
pub struct Camera {
    position: Vector<f64>,
    resolution: [u32; 2],
}

impl Camera {
    pub fn new(resolution: [u32; 2], position: Vector<f64>) -> Camera {
        Camera {
            resolution,
            position,
        }
    }
}

pub struct Scene {
    camera: Camera,
    spheres: Vec<Sphere>,
    light: Vec<Vector<f64>>,
}

impl Scene {
    pub fn new(spheres: Vec<Sphere>, light: Vec<Vector<f64>>, camera: Camera) -> Scene {
        Scene {
            spheres,
            light,
            camera,
        }
    }

    /// Iterates through all objects in scene and calculates
    /// lowest distance to objects.
    fn distance_to_closest(&self, point: &Vector<f64>) -> (f64, u32) {
        let mut min: f64 = f64::MAX;
        let mut id: u32 = 0;

        for sphere in self.spheres.iter() {
            let distance = sphere.sdf(point);
            if distance < min {
                min = distance;
                id = sphere.id;
            }
        }
        (min, id)
    }

    /// Almost same as `distance_to_closest', but ignores objects from the same groupe.
    fn distance_to_closest_offset(&self, point: &Vector<f64>, offset: u32) -> (f64, u32) {
        let mut min: f64 = f64::MAX;
        let mut id: u32 = 0;

        for sphere in self.spheres.iter() {
            if sphere.id == offset {
                continue;
            }
            let distance = sphere.sdf(point);
            if distance < min {
                min = distance;
                id = sphere.id;
            }
        }
        (min, id)
    }

    /// Analog of vector shader.
    ///
    /// Generates Ray for the pixel on screen
    /// and returns color in which coresponding pixel must be coloured.
    pub fn fragment(&self, uv: Vector<f64>) -> Vector<f64> {
        // Vector origin is camera position.
        let ro: Vector<f64> = self.camera.position;

        let mut ray: Ray = Ray::new(uv, ro);
        ray.normalize();
        let color: Vector<f64> = self.ray_march(&ray);
        color
    }

    pub fn render(&self, img: &mut RgbImage) {
        let resx = self.camera.resolution[0] as f64;
        let resy = self.camera.resolution[1] as f64;

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let x: f64 = x as f64;
            let y: f64 = y as f64;

            let uv: Vector<f64> =
                Vector::new((2.0 * x - resx) / resx, (2.0 * y - resy) / resy, 1.0);

            let color: Vector<f64> = self.fragment(uv) * 255.0;
            *pixel = image::Rgb([color.x as u8, color.y as u8, color.z as u8]);
        }
    }

    pub fn ray_march(&self, ray: &Ray) -> Vector<f64> {
        let mut total_distance: f64 = 0.0;
        let step_number: u32 = 100;
        let min_hit_distance: f64 = 0.001;
        let max_distance: f64 = 1000.0;

        let color: Vector<f64> = Vector::new(0.0, 0.0, 0.0);

        for _step in 0..step_number {
            let curent_position: Vector<f64> = ray.position + (ray.direction * total_distance);

            let result: (f64, u32) = self.distance_to_closest(&curent_position);
            let distance_to_closest: f64 = result.0;
            let id: u32 = result.1;

            if distance_to_closest < min_hit_distance {
                let (light, light_angle) = self.light_process(curent_position, id);

                return Vector::new(1.0, 0.0, 0.0) * light_angle * light;
            }

            if distance_to_closest > max_distance {
                break;
            }

            total_distance += distance_to_closest;
        }
        color
    }

    fn light_process(&self, start_position: Vector<f64>, id: u32) -> (f64, f64) {
        let normal = self.calculate_normal(start_position);

        let mut light: f64 = 0.0;
        let mut light_angle: f64 = 0.0;

        for light_source in self.light.iter() {
            let mut direction: Vector<f64> = start_position - *light_source;
            let distance: f64 = direction.len();
            direction = direction.normalize();
            let dot: f64 = normal.dot(&direction);

            if dot <= 0.0 {
                continue;
            }

            if dot > light_angle {
                light_angle = dot;
            }

            let result: f64 = self.cast_light(start_position, direction, distance, id);

            if result > light {
                light = result
            }
        }

        (light, light_angle)
    }

    pub fn cast_light(
        &self,
        start_position: Vector<f64>,
        direction: Vector<f64>,
        distance: f64,
        id: u32,
    ) -> f64 {
        let min_hit_distance: f64 = 0.0001;

        let mut total_distance: f64 = 0.0;
        let direction: Vector<f64> = Vector::new(0.0, 0.0, 0.0) - direction.normalize();

        let mut min_dif: f64 = 1.1;

        while total_distance < distance {
            let curent_position: Vector<f64> = start_position + (direction * total_distance);
            let result: (f64, u32) = self.distance_to_closest_offset(&curent_position, id);
            let distance_to_closest: f64 = result.0;

            let dif: f64 = distance_to_closest - min_hit_distance;

            if dif <= 0.0 {
                return 0.0;
            }

            if dif < min_dif {
                min_dif = dif;
            }

            total_distance += distance_to_closest;
        }
        if min_dif < 1.0 {
            min_dif
        } else {
            1.0
        }
    }

    pub fn calculate_normal(&self, point: Vector<f64>) -> Vector<f64> {
        let small_step: f64 = 0.0001;

        let gradient_x: f64 = self
            .distance_to_closest(&(point + Vector::new(small_step, 0.0, 0.0)))
            .0
            - self
                .distance_to_closest(&(point + Vector::new(-small_step, 0.0, 0.0)))
                .0;

        let gradient_y: f64 = self
            .distance_to_closest(&(point + Vector::new(0.0, small_step, 0.0)))
            .0
            - self
                .distance_to_closest(&(point + Vector::new(0.0, -small_step, 0.0)))
                .0;

        let gradient_z: f64 = self
            .distance_to_closest(&(point + Vector::new(0.0, 0.0, small_step)))
            .0
            - self
                .distance_to_closest(&(point + Vector::new(0.0, 0.0, -small_step)))
                .0;

        let mut normal: Vector<f64> = Vector::new(gradient_x, gradient_y, gradient_z);
        normal = normal.normalize();
        normal * -1.0
    }
}
