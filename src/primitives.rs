use super::linear::Vector;

/// Sphere object
///
/// Round, cool, shinny 
pub struct Sphere {
    /// Radius of the sphere from center to edge
    pub radius: u32,
    /// Object id
    pub id: u32,
    /// Position of sphere in the world
    pub center: Vector<f64>
}

impl Sphere {
    /// Creates a new Sphere object
    ///
    /// # Example
    ///
    /// Create new sphere with radius of 5 and positioned at [0, 0, 0] 
    /// ```
    /// let point: Vector = Vector::new(0.0,0.0,0.0);
    /// let sphere: Sphere = Sphere::new(5, point);
    /// ```
    pub fn new(radius: u32, point: Vector<f64>, id: u32) -> Sphere {
        Sphere{radius: radius, center: point, id: id}
    }

    pub fn sdf(&self, point: &Vector<f64>) -> f64 {
        ((self.center.x - point.x).powi(2) + 
         (self.center.y - point.y).powi(2) +
         (self.center.z - point.z).powi(2)).sqrt() - self.radius as f64
    }

}
