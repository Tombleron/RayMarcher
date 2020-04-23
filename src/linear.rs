use std::ops::{Add,
               Sub,
               Mul,
               Div,
               SubAssign,
               AddAssign,
               MulAssign,
               DivAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vector<T> {
   pub x: T,
   pub y: T,
   pub z: T,
}

impl<T> Vector<T> {

    pub fn new(x: T, y: T, z: T) -> Self {
        Vector{
            x: x,
            y: y,
            z: z,
        }
    }
}

impl Vector<f64> {

    pub fn dot(&self, other: &Vector<f64>) -> f64
    {
        let mut sum = self.x * other.x +
                  self.y * other.y +
                  self.z * other.z;
        //sum /= self.len() * other.len();
        sum 
    }

    pub fn normalize(self) -> Vector<f64> { 
        let normalized = self * (1.0/self.len());
        normalized
    }

    pub fn len(&self) -> f64 

    {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}


impl<T> Add<Vector<T>> for Vector<T> 
    where 
        T: Add<Output = T>
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
        T: Sub<Output = T>
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
        T: Add<Output = T> + Copy
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
        T: Sub<Output = T> + Copy
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
        T: Mul<Output = T> + Copy
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
        T: Div<Output = T> + Copy
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
        T: Div<Output = T> + Copy
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
        T: Mul<Output = T> + Copy
{
    fn mul_assign(&mut self, other: T) {
        *self = Vector {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
        }
    }
}

