//! Simple module to work with vectors
//!
//! Contains several functions to work with vectors,
//! such as dot product, normalization, addition,
//! substraction, multiplication and division of vectors.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Vector type generic.
#[derive(Debug, Copy, Clone)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector { x, y, z }
    }
}

/// Dot product, normalization and length only allowed for Vector<f64> type.
impl Vector<f64> {
    /// Scalar product if you want it to be(Hukutka's request).
    pub fn dot(&self, other: &Vector<f64>) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Generates new vector with same direction,
    /// but lenth is equal to one.
    ///
    /// **Panics if length is zero**
    pub fn normalize(self) -> Vector<f64> {
        self * (1.0 / self.len())
    }

    /// Vector length
    pub fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl<T> Add<Vector<T>> for Vector<T>
where
    T: Add<Output = T>,
{
    type Output = Vector<T>;

    fn add(self, other: Vector<T>) -> Vector<T> {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub<Vector<T>> for Vector<T>
where
    T: Sub<Output = T>,
{
    type Output = Vector<T>;

    fn sub(self, other: Vector<T>) -> Vector<T> {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> AddAssign<Vector<T>> for Vector<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, other: Vector<T>) {
        *self = Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> SubAssign<Vector<T>> for Vector<T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, other: Vector<T>) {
        *self = Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn mul(self, other: T) -> Vector<T> {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T> Div<T> for Vector<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Vector<T>;

    fn div(self, other: T) -> Vector<T> {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T> DivAssign<Vector<T>> for Vector<T>
where
    T: Div<Output = T> + Copy,
{
    fn div_assign(&mut self, other: Vector<T>) {
        *self = Vector {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T> MulAssign<T> for Vector<T>
where
    T: Mul<Output = T> + Copy,
{
    fn mul_assign(&mut self, other: T) {
        *self = Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
