use super::linear::Vector;
use super::primitives::Sphere;
use image::{RgbImage};

/// Ray object
///
/// It's a bird, it's a plane... No, it's a Ray casting through Scene
pub struct Ray {
    /// Dirrection Ray points to
    direction: Vector<f64>,
    /// Current position of Ray in the Scene
    position: Vector<f64>,
}
 
impl Ray {
    /// Creates a new Ray
    /// 
    /// # Example
    ///
    /// Create new Ray facing along X axis at [0,0,0]
    /// ```
    /// let position: Vector = Vector::new(0.0,0.0,0.0);
    /// let direction: Vector = Vector::new(1.0,0.0,0.0);
    /// let ray: Ray = Ray::new(direction, position);
    /// ```
    pub fn new(direction: Vector<f64>, position: Vector<f64>) -> Ray {
        Ray{direction: direction, position: position}
    }
    
    /// Normalizes Ray
    /// 
    /// # Example
    ///
    /// Normalize ray
    /// ```
    /// let position: Vector = Vector::new(0.0,0.0,0.0);
    /// let direction: Vector = Vector::new(1.0,0.0,0.0);
    /// let ray: Ray = Ray::new(direction, position);
    /// ray.normalize();
    /// assert_eq!(ray.len(), 1.0);
    /// ```
    pub fn normalize(&mut self) {
        self.direction = self.direction.normalize();
    }
}

pub struct Camera {
    position: Vector<f64>,
    fov: f64,
    resolution: [u32; 2]
}

impl Camera { 
    pub fn new(resolution: [u32; 2], fov: f64, position: Vector<f64>) -> Camera {
        Camera{resolution: resolution, fov: fov, position: position}
    }
}

pub struct Scene {
    camera: Camera,
    time: f64,
    spheres: Vec<Sphere>,
    light: Vec<Vector<f64>>,
}

impl Scene {
    
    pub fn new(spheres: Vec<Sphere>, light: Vec<Vector<f64>>, time: f64, camera: Camera) -> Scene {
        Scene{spheres: spheres, light: light,camera: camera, time: time}
    }

    pub fn push_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    fn distance_to_closest(&self, point: &Vector<f64>) -> (f64, u32) {
        let mut count: u32 = 0;
        let mut min: f64 = 0.0;
        let mut id: u32 = 0;
        
        
        let displace: f64 = (2.0 * point.x + self.time).sin() * (2.0 * point.y + self.time).sin() * (2.0 * point.z + self.time).sin() * 0.25;

        for sphere in self.spheres.iter() {
            if count == 0 {
                min = sphere.sdf(&point);
                id = sphere.id;
            } else if sphere.sdf(&point) < min {
                min = sphere.sdf(&point);
                id = sphere.id;
            }
            count += 1;
        }
        (min + displace, id)
    }

    fn distance_to_closest_offset(&self, point: &Vector<f64>, offset: u32) -> (f64, u32) {
        let mut count: u32 = 0;
        let mut min: f64 = 1.0;
        let mut id: u32 = 0;
        
        //let displace: f64 = (5.0 * point.x).sin() * (5.0 * point.y).sin() * (5.0 * point.z).sin() * 0.25;

        for sphere in self.spheres.iter() {
            if sphere.id == offset {continue}
            if count == 0 {
                min = sphere.sdf(&point);
                id = sphere.id;
            } else if sphere.sdf(&point) < min {
                min = sphere.sdf(&point);
                id = sphere.id;
            }
            count += 1;
        }
        (min , id)
    }

    
    pub fn fragment(&self, uv: Vector<f64>) -> Vector<f64> {
    
        let ro: Vector<f64> = Vector::new(0.0,0.0,0.0);
        
        let mut ray: Ray = Ray::new(uv, ro);
        ray.normalize();
        //print!("{:.2} ", ray.direction.z);
        let color: Vector<f64> = self.ray_march(&ray);
        color
    }

    pub fn render(&self, img: &mut RgbImage) {
        
        let resx = self.camera.resolution[0] as f64;
        let resy = self.camera.resolution[1] as f64;
        
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            
            let x: f64 = x as f64;
            let y = y as f64;


            let uv: Vector<f64> =  Vector::new(
                                    (2.0 * x - resx)/resx,
                                    (2.0 * y - resy)/resy,
                                    1.0);
            let color: Vector<f64> = self.fragment(uv) * 255.0;
            *pixel = image::Rgb([color.x as u8, 
                                 color.y as u8, 
                                 color.z as u8]);
        }
    }

    pub fn ray_march(&self, ray: &Ray) -> Vector<f64> {
        let mut total_distance: f64 = 0.0;
        let step_number: u32 = 100;
        let min_hit_distance: f64 = 0.001;
        let max_distance: f64 = 1000.0;
        
        let color: Vector<f64> = Vector::new(0.0,0.0,0.0);

        for step in 0..step_number {

            let curent_position: Vector<f64> = ray.position + (ray.direction * total_distance);
            
            let result: (f64, u32) = self.distance_to_closest(&curent_position);
            let distance_to_closest: f64 = result.0;
            let id: u32 = result.1;
            
            if distance_to_closest < min_hit_distance {

                let mut normal = self.calculate_normal(curent_position);
                //normal = normal * 0.5 + Vector::new(0.5,0.5,0.5);
                
                let mut light: f64 = 1.0;
                let mut light_angle: f64 = 0.0;

                for light_source in self.light.iter() {
                    
                    let mut direction: Vector<f64> = curent_position - *light_source;
                    let distance: f64 = direction.len();
                    direction = direction.normalize();
                    let dot: f64 = normal.dot(&direction);

                    if dot <= 0.0 {
                        continue
                    }

                    if dot > light_angle {
                        light_angle = dot;
                    }

                    let result: f64 = self.cast_light(curent_position, direction, distance, id);

                    if result < light {
                        light = result
                    }
                }
               
                return Vector::new(1.0,0.0,0.0) * light_angle * light 
            }

            if distance_to_closest > max_distance {
                break
            }

            total_distance += distance_to_closest;
        }
        color
    }

    pub fn cast_light(&self, start_position: Vector<f64> ,direction: Vector<f64>, distance: f64, id: u32) -> f64 {

        let min_hit_distance: f64 = 0.0001;
        
        let mut total_distance: f64 = 0.0;
        let direction: Vector<f64> = Vector::new(0.0,0.0,0.0)-direction.normalize();
               
        let mut min_dif: f64 = 1.1;

        while total_distance < distance {

            let curent_position: Vector<f64> = start_position + (direction * total_distance);
            let result: (f64, u32) = self.distance_to_closest_offset(&curent_position, id);
            let distance_to_closest: f64 = result.0;
            
            let dif: f64 = distance_to_closest - min_hit_distance;

            if dif <= 0.0 {
                return 0.0
            }

            if dif < min_dif {
                min_dif = dif;
            }
            

            total_distance += distance_to_closest;
        }
        if min_dif < 1.0 {
            return min_dif
        } else {
            1.0
        }
    }



    pub fn calculate_normal(&self, point: Vector<f64>) -> Vector<f64> {
        let small_step: f64 = 0.0001;

        let gradient_x: f64 = self.distance_to_closest(&(point + Vector::new(small_step,0.0,0.0))).0 -
                              self.distance_to_closest(&(point + Vector::new(-small_step,0.0,0.0))).0;

        let gradient_y: f64 = self.distance_to_closest(&(point + Vector::new(0.0,small_step,0.0))).0 -
                              self.distance_to_closest(&(point + Vector::new(0.0,-small_step,0.0))).0;

        let gradient_z: f64 = self.distance_to_closest(&(point + Vector::new(0.0,0.0,small_step))).0 -
                              self.distance_to_closest(&(point + Vector::new(0.0,0.0,-small_step))).0;
        
        let mut normal: Vector<f64> = Vector::new(gradient_x,
                                              gradient_y,
                                              gradient_z);
        normal = normal.normalize();
        normal * -1.0
    }
}

