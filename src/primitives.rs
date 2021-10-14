use super::linear::Vector;

pub struct Sphere {
    pub radius: u32,
    pub id: u32,
    pub center: Vector<f64>,
}

impl Sphere {
    pub fn new(radius: u32, point: Vector<f64>, id: u32) -> Sphere {
        Sphere {
            radius,
            center: point,
            id,
        }
    }
    pub fn sdf(&self, point: &Vector<f64>) -> f64 {
        ((self.center.x - point.x).powi(2)
            + (self.center.y - point.y).powi(2)
            + (self.center.z - point.z).powi(2))
        .sqrt()
            - self.radius as f64
    }
}

pub struct Ray {
    pub direction: Vector<f64>,
    pub position: Vector<f64>,
}

impl Ray {
    pub fn new(direction: Vector<f64>, position: Vector<f64>) -> Ray {
        Ray {
            direction,
            position,
        }
    }

    pub fn normalize(&mut self) {
        self.direction = self.direction.normalize();
    }
}
