//! This module contains usefull objects.
use super::linear::Vector;

/// Sphere is the most easiest object to render.
///
/// Field `id` requiered to group spheres that can
/// interact with each other.
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

    /// Returns signed distance between to spheres.
    ///
    /// Object overlap if this distance less or equal
    /// to zero.
    pub fn sdf(&self, point: &Vector<f64>) -> f64 {
        ((self.center.x - point.x).powi(2)
            + (self.center.y - point.y).powi(2)
            + (self.center.z - point.z).powi(2))
        .sqrt()
            - self.radius as f64
    }
}

/// Ray object represents some vector in space.
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

    /// Normalizes ray direction.
    ///
    /// **Panics if direction's vector length is equal to zero**
    pub fn normalize(&mut self) {
        self.direction = self.direction.normalize();
    }
}
