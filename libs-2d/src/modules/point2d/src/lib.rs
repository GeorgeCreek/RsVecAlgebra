// Reuse Vector2d from your vector2d module
use vector2d::Vector2d;
use std::ops::Neg;
use std::hash::{Hash, Hasher};
use num_traits::{Float, Zero}; // Add this import
use std::ops::AddAssign;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point2d<T>(pub Vector2d<T>);

// Modified implementation with required trait bounds
impl<T: Copy + Clone + Zero + std::ops::Mul + PartialOrd> Point2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Point2d(Vector2d::new(x, y))
    }
    pub fn x(&self) -> T {
        self.0.x
    }
    pub fn set_x(&mut self, x: T) {
        self.0.x = x;
    }
    pub fn y(&self) -> T {
        self.0.y
    }
    pub fn set_y(&mut self, y: T) {
        self.0.y = y;
    }
    pub fn with_x_y(x: T, y: T) -> Self {
        Point2d(Vector2d::new(x, y))
    }
    pub fn with_x(x: T) -> Self {
        Point2d(Vector2d::new(x, T::zero()))
    }
    pub fn with_y(y: T) -> Self {
        Point2d(Vector2d::new(T::zero(), y))
    }
    pub fn as_mut_vector(&mut self) -> &mut Vector2d<T> {
        &mut self.0
    }
    pub fn as_vector(&self) -> &Vector2d<T> {
        &self.0
    }
    pub fn get(&self) -> (T, T) {
        (self.x(), self.y())
    }
    pub fn set(&mut self, x: T, y: T) {
        self.0.x = x;
        self.0.y = y;
    }
    pub fn to_tuple(&self) -> (T, T) {
        (self.x(), self.y())
    }
    pub fn to_vector(&self) -> Vector2d<T> {
        self.0
    }
    pub fn from_vector(vector: Vector2d<T>) -> Self {
        Point2d(vector)
    }
}
impl<T: Float + Copy + Clone + Zero + std::ops::Mul<Output = T> + PartialOrd> Point2d<T> {
    pub fn length(&self) -> T {
        (self.x() * self.x() + self.y() * self.y()).sqrt()
    }
    pub fn length_squared(&self) -> T {
        self.x() * self.x() + self.y() * self.y()
    }
    pub fn angle(&self) -> T {
        self.y().atan2(self.x())
    }
    pub fn angle_between(&self, other: &Self) -> T {
        let dot_product = self.x() * other.x() + self.y() * other.y();
        let magnitude_self = self.length();
        let magnitude_other = other.length();
        (dot_product / (magnitude_self * magnitude_other)).acos()
    }
    pub fn distance_to(&self, other: &Self) -> T {
        ((self.x() - other.x()).powi(2) + (self.y() - other.y()).powi(2)).sqrt()
    }
    pub fn distance_squared_to(&self, other: &Self) -> T {
        (self.x() - other.x()).powi(2) + (self.y() - other.y()).powi(2)
    }
    pub fn dot(&self, other: &Self) -> T {
        self.x() * other.x() + self.y() * other.y()
    }
    pub fn cross(&self, other: &Self) -> T {
        self.x() * other.y() - self.y() * other.x()
    }
    pub fn normalized(&self) -> Self {
        let length = self.length();
        if length == T::zero() {
            *self
        } else {
            Point2d::new(self.x() / length, self.y() / length)
        }
    }
    pub fn unit_vector(&self) -> Self {
        let length = self.length();
        if length == T::zero() {
            *self
        } else {
            Point2d::new(self.x() / length, self.y() / length)
        }
    }
    pub fn is_unit_vector(&self) -> bool {
        (self.length() - T::one()).abs() < T::epsilon()
    }
    pub fn is_equal(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y()
    }
    pub fn is_approx_equal(&self, other: &Self) -> bool {
        (self.x() - other.x()).abs() < T::epsilon() && (self.y() - other.y()).abs() < T::epsilon()
    }
    pub fn is_collinear(&self, other: &Self) -> bool {
        self.cross(other).abs() < T::epsilon()
    }
    pub fn is_parallel(&self, other: &Self) -> bool {
        self.cross(other).abs() < T::epsilon()
    }
    pub fn is_perpendicular(&self, other: &Self) -> bool {
        self.dot(other).abs() < T::epsilon()
    }
    pub fn is_orthogonal(&self, other: &Self) -> bool {
        self.dot(other).abs() < T::epsilon()
    }
    pub fn is_unit(&self) -> bool {
        (self.length() - T::one()).abs() < T::epsilon()
    }
    pub fn is_normalized(&self) -> bool {
        (self.length() - T::one()).abs() < T::epsilon()
    }
    pub fn is_non_zero(&self) -> bool {
        self.x() != T::zero() || self.y() != T::zero()
    }
    pub fn is_not_null(&self) -> bool {
        self.x() != T::zero() || self.y() != T::zero()
    }
    pub fn is_positive(&self) -> bool {
        self.x() > T::zero() && self.y() > T::zero()
    }
    pub fn is_negative(&self) -> bool {
        self.x() < T::zero() && self.y() < T::zero()
    }
    pub fn is_zero(&self) -> bool {
        self.x() == T::zero() && self.y() == T::zero()
    }
    pub fn is_null(&self) -> bool {
        self.x() == T::zero() && self.y() == T::zero()
    }
}
impl<T> Point2d<T>
where T: Copy + Into<f64> + std::ops::Sub<Output = T>,
{
    pub fn distance(&self, other: &Self) -> f64 {
        let dx = (self.0.x - other.0.x).into();
        let dy = (self.0.y - other.0.y).into();
        (dx * dx + dy * dy).sqrt()
    }
}

impl<T> Point2d<T>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Div<Output = T> + From<u8> + num_traits::Zero + std::ops::Mul + std::cmp::PartialOrd,
{
    pub fn midpoint(&self, other: &Self) -> Self {
        Point2d::new(
            (self.0.x + other.0.x) / T::from(2),
            (self.0.y + other.0.y) / T::from(2),
        )
    }
}
impl Point2d<f64> {
    pub fn rotate(&self, angle: f64) -> Self {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();
        Point2d::new(
            self.x() * cos_theta - self.y() * sin_theta,
            self.x() * sin_theta + self.y() * cos_theta,
        )
    }
}
impl<T> Point2d<T>
where
    T: Copy + std::ops::Neg<Output = T> + num_traits::Zero + std::ops::Mul + std::cmp::PartialOrd,
{
    pub fn reflect_x(&self) -> Self {
        Point2d::new(self.0.x, -self.0.y)
    }

    pub fn reflect_y(&self) -> Self {
        Point2d::new(-self.0.x, self.0.y)
    }

    pub fn reflect_origin(&self) -> Self {
        Point2d::new(-self.0.x, -self.0.y)
    }
}
// Fixed implementation with proper trait bounds
impl<T> std::ops::Add<Vector2d<T>> for Point2d<T>
where
    T: Copy + Clone + Zero + std::ops::Mul + PartialOrd + std::ops::Add<Output = T>,
{
    type Output = Point2d<T>;
    fn add(self, rhs: Vector2d<T>) -> Self::Output {
        Point2d(Vector2d::new(self.x() + rhs.x, self.y() + rhs.y))
    }
}
impl<T> AddAssign for Point2d<T>
where
    T: Copy + Clone + Zero + std::ops::Mul + PartialOrd + std::ops::Add<Output = T> + std::ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Point2d<T>) {
        self.0.x += rhs.x();
        self.0.y += rhs.y();
    }
}

impl<T> Neg for Point2d<T>
where
    T: Neg<Output = T> + Copy + Clone + Zero + std::ops::Mul + PartialOrd,
{
    type Output = Point2d<T>;
    fn neg(self) -> Self::Output {
        Point2d(Vector2d::new(-self.x(), -self.y()))
    }
}

impl<T: Copy + Clone + Zero + std::ops::Mul + PartialOrd> PartialOrd for Point2d<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.x().partial_cmp(&other.x()) {
            Some(std::cmp::Ordering::Equal) => self.y().partial_cmp(&other.y()),
            ord => ord,
        }
    }
}
// Change this implementation
impl<T: Default + Copy + Zero + std::ops::Mul + std::cmp::PartialOrd> Default for Point2d<T> {
    fn default() -> Self {
        Point2d(Vector2d::new(T::default(), T::default()))
    }
}


impl<T: Hash + Copy + Clone + Zero + std::ops::Mul + PartialOrd> Hash for Point2d<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x().hash(state);
        self.y().hash(state);
    }
}

// Example: Add two Point2d instances to get a new Point2d
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use vector2d::Vector2d;

    #[test]
    fn test_point2d_creation() {
        let point = Point2d::new(3, 4);
        assert_eq!(point.x(), 3);
        assert_eq!(point.y(), 4);
    }

    #[test]
    fn test_point2d_add_vector2d() {
        let point = Point2d::new(1, 2);
        let vector = Vector2d::new(3, 4);
        let result = point + vector;
        assert_eq!(result.x(), 4);
        assert_eq!(result.y(), 6);
    }

    #[test]
    fn test_point2d_negation() {
        let point = Point2d::new(5, -7);
        let negated = -point;
        assert_eq!(negated.x(), -5);
        assert_eq!(negated.y(), 7);
    }

    #[test]
    fn test_point2d_default() {
        let default_point: Point2d<i32> = Point2d::default();
        assert_eq!(default_point.x(), 0);
        assert_eq!(default_point.y(), 0);
    }

    #[test]
    fn test_point2d_partial_cmp() {
        let point1 = Point2d::new(1, 2);
        let point2 = Point2d::new(1, 3);
        assert!(point1 < point2);
    }
}