use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// Default vector operations

impl Add<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3D> for f64 {
    type Output = Vector3D;
    fn mul(self, rhs: Vector3D) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl SubAssign for Vector3D {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl PartialEq for Vector3D {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= f64::EPSILON
            && (self.y - other.y).abs() <= f64::EPSILON
            && (self.z - other.z).abs() <= f64::EPSILON
    }
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// The length squared might come in handy!
    pub fn length_squared(&self) -> f64 {
        // QUESTION: is this faster than the below?
        // self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
        self.dot(&self)
    }

    /// Length of vector.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Dot product of two vectors.
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Dot product of two vectors.
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Reduce length to 1.
    pub fn normalized(&self) -> Self {
        *self / self.length()
    }
}

// Point +/- Vector = Point

impl Add<Point3D> for Vector3D {
    type Output = Point3D;
    fn add(self, rhs: Point3D) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vector3D> for Point3D {
    type Output = Point3D;
    fn add(self, rhs: Vector3D) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Point3D> for Vector3D {
    type Output = Point3D;
    fn sub(self, rhs: Point3D) -> Self::Output {
        Self::Output {
            x: rhs.x - self.x,
            y: rhs.y - self.y,
            z: rhs.z - self.z,
        }
    }
}

impl Sub<Vector3D> for Point3D {
    type Output = Point3D;
    fn sub(self, rhs: Vector3D) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// Point - Point = Vector
// Point + Point doesn't make logical sense

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl PartialEq for Point3D {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= f64::EPSILON
            && (self.y - other.y).abs() <= f64::EPSILON
            && (self.z - other.z).abs() <= f64::EPSILON
    }
}

impl Sub<Point3D> for Point3D {
    type Output = Vector3D;
    fn sub(self, rhs: Point3D) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vx() -> Vector3D {
        Vector3D::new(1.0, 0.0, 0.0)
    }

    fn vy() -> Vector3D {
        Vector3D::new(0.0, 1.0, 0.0)
    }

    fn vz() -> Vector3D {
        Vector3D::new(0.0, 0.0, 1.0)
    }

    fn vo() -> Vector3D {
        Vector3D::new(0.0, 0.0, 0.0)
    }

    fn px() -> Point3D {
        Point3D::new(1.0, 0.0, 0.0)
    }

    fn py() -> Point3D {
        Point3D::new(0.0, 1.0, 0.0)
    }

    fn pz() -> Point3D {
        Point3D::new(0.0, 0.0, 1.0)
    }

    fn po() -> Point3D {
        Point3D::new(0.0, 0.0, 0.0)
    }

    #[cfg(test)]
    mod vector_tests {
        use super::*;

        #[test]
        fn vector_addition() {
            assert_eq!(vx() + vy(), Vector3D::new(1.0, 1.0, 0.0));
            assert_eq!(vx() + vo(), vx());
        }

        #[test]
        fn vector_assign_addition() {
            let mut v = vx();
            v += vy();
            assert_eq!(v, vx() + vy());
        }

        #[test]
        fn vector_subtraction() {
            assert_eq!(vx() - vy(), Vector3D::new(1.0, -1.0, 0.0));
            assert_eq!(vx() - vo(), vx());
            assert_eq!(vo() - vx(), -1.0 * vx());
        }

        #[test]
        fn vector_assign_subtraction() {
            let mut v = vx();
            v -= vy();
            assert_eq!(v, vx() - vy());
        }

        #[test]
        fn vector_dot() {
            assert!(vx().dot(&vy()) - 0.0 <= f64::EPSILON);
            assert!(vx().dot(&vo()) - 0.0 <= f64::EPSILON);
            assert!(vo().dot(&vx()) - 0.0 <= f64::EPSILON);
            assert!(vx().dot(&vx()) - 1.0 <= f64::EPSILON);
        }

        #[test]
        fn vector_cross() {
            assert_eq!(vx().cross(&vx()), vo());
            assert_eq!(vx().cross(&vy()), vz());
            assert_eq!(vx().cross(&vo()), vo());
        }

        #[test]
        fn vector_size() {
            assert!(vx().length_squared() - 1.0 <= f64::EPSILON);
            assert!(vx().length() - 1.0 <= f64::EPSILON);
            assert!((2.0 * vx()).length() - 2.0 <= f64::EPSILON);
            assert!(vo().length() <= f64::EPSILON);
            assert_eq!((2.0 * vz()).normalized(), vz());
        }

        #[test]
        fn vector_equality() {
            assert_eq!(vx(), vx());
        }
    }

    #[cfg(test)]
    mod point_tests {
        use super::*;

        #[test]
        fn point_subtraction() {
            assert_eq!(px() - po(), vx());
            assert_eq!(po() - px(), -1.0 * vx());
            assert_eq!(py() - py(), vo());
        }

        #[test]
        fn point_equality() {
            assert_eq!(px(), px());
        }
    }

    #[cfg(test)]
    mod point_vector_integration_tests {
        use super::*;

        #[test]
        fn point_vector_addition() {
            assert_eq!(po() + vz(), pz());
            assert_eq!(vz() + po(), pz());
            assert_eq!(po() + vo(), po());
            assert_eq!(px() + vo(), px());
        }

        #[test]
        fn point_vector_subtraction() {
            assert_eq!(po() - vz(), Point3D::new(0.0, 0.0, -1.0));
            assert_eq!(vz() - po(), Point3D::new(0.0, 0.0, -1.0));
            assert_eq!(po() - vo(), po());
            assert_eq!(px() - vo(), px());
        }
    }
}
