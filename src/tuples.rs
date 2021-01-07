use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub struct Float(f64);

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 0.00001
    }
}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            self.0.partial_cmp(&other.0)
        }
    }
}

impl std::ops::Add for Float {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Float(self.0 + other.0)
    }
}

impl std::ops::Sub for Float {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Float(self.0 - other.0)
    }
}

impl std::ops::Mul for Float {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Float(self.0 * other.0)
    }
}

impl std::ops::Div for Float {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Float(self.0 / other.0)
    }
}

impl std::ops::Neg for Float {
    type Output = Float;
    fn neg(self) -> Self::Output {
        Float(-self.0)
    }
}

impl From<f64> for Float {
    fn from(a: f64) -> Self {
        Float(a)
    }
}

impl Float {
    pub fn abs(self) -> Float {
        Float(self.0.abs())
    }

    pub fn sqr(self) -> Float {
        Float(self.0 * self.0)
    }

    pub fn sqrt(self) -> Float {
        Float(self.0.sqrt())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tuple {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub w: Float,
}

impl std::ops::Add for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl std::ops::Sub for Tuple {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl std::ops::Mul<Float> for Tuple {
    type Output = Self;
    fn mul(self, other: Float) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl std::ops::Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Tuple {
    pub fn new(x: Float, y: Float, z: Float, w: Float) -> Self {
        Self { x, y, z, w }
    }

    pub fn is_point(self) -> bool {
        self.w == Float(1.0)
    }
    pub fn is_vector(self) -> bool {
        self.w == Float(0.0)
    }

    pub fn magnitude(self) -> Float {
        (self.x.sqr() + self.y.sqr() + self.z.sqr() + self.w.sqr()).sqrt()
    }

    pub fn normalize(self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }

    pub fn dot(self, other: Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(self, other: Self) -> Tuple {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: Float(0.0),
        }
    }
}

#[macro_export]
macro_rules! tuple {
    ($x:expr, $y: expr, $z: expr, $w: expr) => {
        Tuple::new(Float($x), Float($y), Float($z), Float($w))
    };
}
#[macro_export]
macro_rules! point {
    ($x:expr, $y: expr, $z: expr) => {
        tuple!($x, $y, $z, 1.0)
    };
}
#[macro_export]
macro_rules! vector {
    ($x:expr, $y: expr, $z: expr) => {
        tuple!($x, $y, $z, 0.0)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_tuple_with_w_1_is_a_point() {
        let a = tuple!(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, Float(4.3));
        assert_eq!(a.y, Float(-4.2));
        assert_eq!(a.z, Float(3.1));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }
    #[test]
    fn a_tuple_with_w_0_is_a_vector() {
        let a = tuple!(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, Float(4.3));
        assert_eq!(a.y, Float(-4.2));
        assert_eq!(a.z, Float(3.1));
        assert!(!a.is_point());
        assert!(a.is_vector());
    }
    #[test]
    fn point_creates_tuples_with_w_1() {
        let p = point!(4.0, -4.0, 3.0);
        assert_eq!(p, tuple!(4.0, -4.0, 3.0, 1.0));
    }
    #[test]
    fn vector_creates_tuples_with_w_0() {
        let p = vector!(4.0, -4.0, 3.0);
        assert_eq!(p, tuple!(4.0, -4.0, 3.0, 0.0));
    }
    #[test]
    fn adding_two_tuples() {
        let a1 = tuple!(3.0, -2.0, 5.0, 1.0);
        let a2 = tuple!(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a1 + a2, tuple!(1.0, 1.0, 6.0, 1.0));
    }
    #[test]
    fn subtracting_two_tuples() {
        let p1 = point!(3.0, 2.0, 1.0);
        let p2 = point!(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, vector!(-2.0, -4.0, -6.0));
    }
    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = point!(3.0, 2.0, 1.0);
        let v = vector!(5.0, 6.0, 7.0);
        assert_eq!(p - v, point!(-2.0, -4.0, -6.0));
    }
    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector!(3.0, 2.0, 1.0);
        let v2 = vector!(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, vector!(-2.0, -4.0, -6.0));
    }
    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = vector!(0.0, 0.0, 0.0);
        let v = vector!(1.0, -2.0, 3.0);
        assert_eq!(zero - v, vector!(-1.0, 2.0, -3.0));
    }
    #[test]
    fn negating_a_tuple() {
        let a = tuple!(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, tuple!(-1.0, 2.0, -3.0, 4.0));
    }
    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = tuple!(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * Float(3.5), tuple!(3.5, -7.0, 10.5, -14.0));
    }
    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = tuple!(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * Float(0.5), tuple!(0.5, -1.0, 1.5, -2.0));
    }
    #[test]
    fn computing_the_magnitude_of_vector_1_0_0() {
        let v = vector!(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), Float(1.0));
    }
    #[test]
    fn computing_the_magnitude_of_vector_0_1_0() {
        let v = vector!(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), Float(1.0));
    }
    #[test]
    fn computing_the_magnitude_of_vector_0_0_1() {
        let v = vector!(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), Float(1.0));
    }
    #[test]
    fn computing_the_magnitude_of_vector_1_2_3() {
        let v = vector!(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), Float(14.0).sqrt());
    }
    #[test]
    fn computing_the_magnitude_of_vector_n1_n2_n3() {
        let v = vector!(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), Float(14.0).sqrt());
    }
    #[test]
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let v = vector!(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), vector!(1.0, 0.0, 0.0));
    }
    #[test]
    fn normalizing_vector_1_2_3_gives_1_2_3() {
        let v = vector!(1.0, 2.0, 3.0);
        assert_eq!(v.normalize(), vector!(0.26726, 0.53452, 0.80178));
    }
    #[test]
    fn the_dot_product_of_two_tuples() {
        let a = vector!(1.0, 2.0, 3.0);
        let b = vector!(2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), Float(20.0));
    }
    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = vector!(1.0, 2.0, 3.0);
        let b = vector!(2.0, 3.0, 4.0);
        assert_eq!(a.cross(b), vector!(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(a), vector!(1.0, -2.0, 1.0));
    }
}
