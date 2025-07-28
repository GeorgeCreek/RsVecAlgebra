use std::ops::{Add, Sub, Mul, Div, Neg };
use std::cmp::PartialOrd;

use num_traits::{Float, NumCast, Signed};
use::num_traits::identities::Zero;
///use num_traits::real::Real;
///use std::io::{self, Read};
#[derive(Clone)]
pub struct Vector2d<T> { //<T: Float> {
    pub x: T,
    pub y: T,
}
impl<T: Copy + Clone + Zero + std::ops::Mul + PartialOrd> Vector2d<T> {
    /// Creates a new `Vector2d` with the specified x and y components.
    ///
    /// # Arguments
    /// - `x`: The x-component of the vector.
    /// - `y`: The y-component of the vector.
    ///
    /// # Returns
    /// A new `Vector2d` instance with the given components.
    ///
    /// # Example
    /// ```rust
    /// use vector2d::Vector2d;
    /// let vector = Vector2d::new(3.0, 4.0);
    /// assert_eq!(vector.x, 3.0);
    /// assert_eq!(vector.y, 4.0);
    /// ```
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn null() -> Self {
        Self { x: Zero::zero(), y: Zero::zero() }
    }
    pub fn is_null(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
    pub fn is_not_null(&self) -> bool {
        !self.is_null()
    }
    pub fn is_zero(&self) -> bool {
        self.is_null()
    }
    pub fn is_not_zero(&self) -> bool {
        self.is_not_null()
    }
    /// Converts a `Vector2d<U>` to a `Vector2d<T>`, where `U` can be converted into `T`.
    /// This is useful for converting between different numeric types.
    /// # Arguments
    /// - `src`: The source vector of type `Vector2d<U>`.
    /// # Returns
    /// A new `Vector2d<T>` with the components converted from `U` to `T`.
    /// # Example
    /// ```rust
    /// use vector2d::Vector2d;
    /// let src = Vector2d::new(3.0_f32, 4.0_f32);
    /// let vector: Vector2d<i32> = Vector2d::from_vector2d(src);
    /// assert_eq!(vector.x, 3);
    /// assert_eq!(vector.y, 4);
    /// ```
    /*
    pub fn from_vector2d<U: Into<T> + Copy + Clone>(src: Vector2d<U>) -> Vector2d<T> {
        Vector2d {
            x: src.x.into(),
            y: src.y.into(),
        }
    }
    */
    /// Converts `self` into a `Vector2d<U>`, where `U` can be converted from `T`.
    pub fn into_vector2d<U: From<T>>(self) -> Vector2d<U> {
        Vector2d {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}
impl<T: Copy> Copy for Vector2d<T> {
    // This trait is empty, as Copy does not require any methods.
    // It simply indicates that the type can be copied without moving.
}
impl<T: Clone> Clone for Vector2d<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}
impl<T: Copy + Clone + Zero + std::ops::Mul + PartialOrd> Vector2d<T> {
    // ... other methods remain the same ...

    pub fn from_vector2d<U>(src: Vector2d<U>) -> Vector2d<T>
    where
        U: Copy + Clone + NumCast,
        T: NumCast + Zero
    {
        Vector2d {
            x: NumCast::from(src.x).unwrap_or_else(T::zero),
            y: NumCast::from(src.y).unwrap_or_else(T::zero),
        }
    }
}

impl<T: Copy + Clone + Zero + std::ops::Mul<Output = T> + std::ops::Sub<Output = T> + std::ops::Div<Output = T>> Vector2d<T> {
    /// Returns a new vector that is the null vector (0, 0).
    pub fn null_vector() -> Self {
        Self {
            x: Zero::zero(),
            y: Zero::zero(),
        }
    }
    /// Returns a new vector that is the null vector (0, 0).
    pub fn zero_vector() -> Self {
        Self::null_vector()
    }
    pub fn add<U: Into<T> + Copy + Clone>(&self, other: &Vector2d<U>) -> Self {
        Self {
            x: self.x + other.x.into(),
            y: self.y + other.y.into(),
        }
    }
    /// Returns a new vector that is the difference of `self` and `other`.
    pub fn sub<U: Into<T> + Copy + Clone>(&self, other: &Vector2d<U>) -> Self {
        Self {
            x: self.x - other.x.into(),
            y: self.y - other.y.into(),
        }
    }
    pub fn mul<U: Into<T> + Copy + Clone>(&self, other: &Vector2d<U>) -> Self {
        Self {
            x: self.x * other.x.into(),
            y: self.y * other.y.into(),
        }
    }
    pub fn div<U: Into<T> + Copy + Clone>(&self, other: &Vector2d<U>) -> Self {
        Self {
            x: self.x / other.x.into(),
            y: self.y / other.y.into(),
        }
    }
    pub fn add_scalar(&self, scalar: T) -> Self {
        Self {
            x: self.x + scalar,
            y: self.y + scalar,
        }
    }
    pub fn sub_scalar(&self, scalar: T) -> Self {
        Self {
            x: self.x - scalar,
            y: self.y - scalar,
        }
    }
    pub fn mul_scalar(&self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
    pub fn div_scalar(&self, scalar: T) -> Self {
        if scalar.is_zero() {
            panic!("Division by zero is not allowed in Vector2d");
        }
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
    pub fn hadamard_product(&self, other: &Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
impl<T> Mul<T> for Vector2d<T>
where
    T: Copy + Clone + Mul<Output = T>,
{
    type Output = Vector2d<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Vector2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Mul<T> for &Vector2d<T>
where
    T: Copy + Clone + Mul<Output = T>,
{
    type Output = Vector2d<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Vector2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl<T, O> Add<&Vector2d<T>> for &Vector2d<T>
where
    T: Add<T, Output=O> + Copy + Clone,
{
    type Output = Vector2d<O>;
    fn add(self, rhs: &Vector2d<T>) -> Self::Output {
        Vector2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<T, O> Sub<&Vector2d<T>> for &Vector2d<T>
where
    T: Sub<T, Output=O> + Copy + Clone,
{
    type Output = Vector2d<O>;
    fn sub(self, rhs: &Vector2d<T>) -> Self::Output {
        Vector2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<T, O> Mul<&Vector2d<T>> for &Vector2d<T>
where
    T: Mul<T, Output=O> + Copy + Clone,
{
    type Output = Vector2d<O>;
    fn mul(self, rhs: &Vector2d<T>) -> Self::Output {
        Vector2d {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl<T, O> Div<&Vector2d<T>> for &Vector2d<T>
where
    T: Div<T, Output=O> + Copy + Clone,
{
    type Output = Vector2d<O>;
    fn div(self, rhs: &Vector2d<T>) -> Self::Output {
        Vector2d {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: Default> Default for Vector2d<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T: Default> Vector2d<T> {
    pub fn x_axis(self) -> Self {
        Self {
            x: self.x,
            y: T::default(),
        }
    }

    pub fn y_axis(self) -> Self {
        Self {
            x: T::default(),
            y: self.y,
        }
    }
}

impl<T: Copy + Clone + std::default::Default> Vector2d<T> {
    pub fn x(&self) -> T {
        self.x
    }
    pub fn y(&self) -> T {
        self.y
    }
    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }

    /// Returns a new vector with the x component set to the value of `x` and the y component set to the value of `y`.
    pub fn with_x_y(x: T, y: T) -> Self {
        Self { x, y }
    }
    /// Returns a new vector with the x component set to the value of `x` and the y component set to the default value.
    pub fn with_x(x: T) -> Self {
        Self { x, y: Default::default() }
    }
    /// Returns a new vector with the y component set to the value of `y` and the x component set to the default value.
    pub fn with_y(y: T) -> Self {
        Self { x: Default::default(), y }
    }
}
// Implementing the Add and Sub traits for Vector2d
impl<T, O> Add<Vector2d<T>> for Vector2d<T>
where T: Add<T, Output=O> + Copy + Clone,
{
    type Output = Vector2d<O>;
    fn add(self, rhs: Vector2d<T>) -> Self::Output {
        Vector2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<T, O> Sub<Vector2d<T>> for Vector2d<T>
where T: Sub<T, Output=O> + Copy + Clone,
{
    type Output = Vector2d<O>;
    fn sub(self, rhs: Vector2d<T>) -> Self::Output {
        Vector2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<T, O> Mul<Vector2d<T>> for Vector2d<T>
where T: Mul<T, Output=O> + Copy + Clone,
{
    type Output = Vector2d<O>;
    fn mul(self, rhs: Vector2d<T>) -> Self::Output {
        Vector2d {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl<T, O> Div<Vector2d<T>> for Vector2d<T>
where T: Div<T, Output=O> + Copy + Clone,
{
    type Output = Vector2d<O>;
    fn div(self, rhs: Vector2d<T>) -> Self::Output {
        Vector2d {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

// New implementation block for comparison methods that require PartialOrd
impl<T: Copy + Clone + Zero + std::ops::Mul + PartialOrd> Vector2d<T> {
    pub fn is_positive(&self) -> bool {
        self.x > Zero::zero() && self.y > Zero::zero()
    }
    pub fn is_negative(&self) -> bool {
        self.x < Zero::zero() && self.y < Zero::zero()
    }
    pub fn is_positive_x(&self) -> bool {
        self.x > Zero::zero()
    }

    pub fn is_positive_y(&self) -> bool {
        self.y > Zero::zero()
    }

    pub fn is_negative_x(&self) -> bool {
        self.x < Zero::zero()
    }
    pub fn is_negative_y(&self) -> bool {
        self.y < Zero::zero()
    }
}

impl<T: Float + Signed> Vector2d<T> {
    pub fn sqrt(&self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
        }
    }
    pub fn abs(&self) -> Self
    where
        T: Signed,
    {
        // Ensure T implements Signed for abs to work correctly
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
    pub fn pow2(&self) -> Self {
        Self {
            x: self.x * self.x,
            y: self.y * self.y,
        }
    }

    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn length_squared(&self) -> T
    where
        <T as Mul>::Output: Add
    {
        self.x * self.x + self.y * self.y
    }
    pub fn angle(&self) -> T {
        self.y.atan2(self.x)
    }
    pub fn angle_degrees(&self) -> T {
        self.angle() * T::from(180.0 / std::f32::consts::PI).unwrap()
    }
    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len.is_zero() {
            Self::null()
        } else {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        }
    }
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }
    pub fn cross(&self, other: &Self) -> T {
        self.x * other.y - self.y * other.x
    }
    pub fn distance_to(&self, other: &Self) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    pub fn manhattan_magnitude(&self) -> T {
        self.x.abs() + self.y.abs()
    }
    pub fn manhattan_distance(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
    pub fn midpoint_to(&self, other: &Self) -> Self {
        Self {
            x: (self.x + other.x) / T::from(2).unwrap(),
            y: (self.y + other.y) / T::from(2).unwrap(),
        }
    }
    pub fn angle_to(&self, other: &Self) -> T {
        let dot_product = self.dot(other);
        let magnitude_self = self.length();
        let magnitude_other = other.length();
        if magnitude_self.is_zero() || magnitude_other.is_zero() {
            T::zero()
        } else {
            (dot_product / (magnitude_self * magnitude_other)).acos()
        }
    }
    pub fn angle_between(&self, other: &Self) -> T {
        let angle_self = self.angle();
        let angle_other = other.angle();
        (angle_other - angle_self).abs()
    }
    pub fn angle_between_degrees(&self, other: &Self) -> T {
        let angle_radians = self.angle_between(other);
        angle_radians * T::from(180.0 / std::f32::consts::PI).unwrap()
    }
    pub fn angle_between_is_clockwise(&self, other: &Self) -> T {
        let angle_self = self.angle();
        let angle_other = other.angle();
        let angle_diff = angle_other - angle_self;
        if angle_diff < T::zero() {
            T::one() // Clockwise
        } else {
            T::zero() // Counter-clockwise
        }
    }
    pub fn angle_from_x_axis_positive(&self) -> T {
        let mut angle = T::zero().atan2(T::one()) - self.y.atan2(self.x);
        angle = angle * T::from(180.0 / std::f32::consts::PI).unwrap();
        if angle < T::zero() {
            angle + T::from(360.0).unwrap()
        } else {
            angle
        }
    }
    pub fn angle_from_x_axis_negative(&self) -> T {
        let mut angle = T::zero().atan2(T::one()) - self.y.atan2(self.x);
        angle = angle * T::from(180.0 / std::f32::consts::PI).unwrap();
        if angle > T::zero() {
            angle - T::from(360.0).unwrap()
        } else {
            angle
        }
    }
    /// Linear interpolation between two vectors.
    pub fn lerp(&self, other: &Self, t: T) -> Self {
        if t <= T::zero() {
            self.clone()
        } else if t >= T::one() {
            other.clone()
        } else {
            self * (T::one() - t) + other * t
        }
    }
    /// Clamps the magnitude of the vector to a maximum length.
    /// If the vector's length exceeds `max_length`, it is scaled down to that length.
    /// If the vector's length is less than or equal to `max_length`, it remains unchanged.
    pub fn clamp_magnitude(self, max_length: T) -> Self {
        let len = self.length();
        if len > max_length {
            let norm = self.normalized();
            Self {
                x: norm.x * max_length,
                y: norm.y * max_length,
            }
        } else {
            self
        }
    }
    pub fn clamp_components(&self, min: T, max: T) -> Self {
        Self {
            x: self.x.max(min).min(max),
            y: self.y.max(min).min(max),
        }
    }
    pub fn translate(&mut self, dx: T, dy: T) {
        if dx.is_zero() && dy.is_zero() {
            return; // No translation needed
        }
        self.x = self.x + dx;
        self.y = self.y + dy;
    }
    pub fn translate_x(&mut self, dx: T) {
        if dx.is_zero() {
            return; // No translation needed
        }
        self.x = self.x + dx;
    }
    pub fn scale(&self, factor: T) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
    pub fn translate_y(&mut self, dy: T) {
        if dy.is_zero() {
            return; // No translation needed
        }
        self.y = self.y + dy;
    }
    pub fn rotate(&self, angle: T) -> Self {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        Self {
            x: self.x * cos_angle - self.y * sin_angle,
            y: self.x * sin_angle + self.y * cos_angle,
        }
    }
    pub fn reflect(&self, normal: &Self) -> Self {
        let dot_product = self.dot(normal);
        Self {
            x: self.x - T::from(2.0).unwrap() * dot_product * normal.x,
            y: self.y - T::from(2.0).unwrap() * dot_product * normal.y,
        }
    }
    pub fn projection_length(&self, other: &Self) -> T {
        self.dot(other) / other.length()
    }
    pub fn reflect_across(&self, axis: &Self) -> Self {
        let projection = self.project_onto(axis);
        projection * T::from(2.0).unwrap() - self.clone()
    }
    pub fn project_onto(&self, other: &Self) -> Self {
        let dot_product = self.dot(other);
        let length_squared = other.length_squared();
        Self {
            x: other.x * dot_product / length_squared,
            y: other.y * dot_product / length_squared,
        }
    }
    pub fn perpendicular_to(&self, other: &Self) -> Self {
        let dot_product = self.dot(other);
        let length_squared = other.length_squared();
        Self {
            x: other.y * dot_product / length_squared,
            y: -other.x * dot_product / length_squared,
        }
    }
    pub fn set_neg(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
    pub fn set_polar(&mut self, radius: T, angle: T) {
        self.x = radius * angle.cos();
        self.y = radius * angle.sin();
    }
    pub fn set_polar_degrees(&mut self, radius: T, angle: T) {
        let angle_rad = angle * T::from(std::f32::consts::PI / 180.0).unwrap();
        self.x = radius * angle_rad.cos();
        self.y = radius * angle_rad.sin();
    }
    pub fn get_polar(&self) -> (T, T) {
        let radius = self.length();
        let angle = self.angle();
        (radius, angle)
    }
    pub fn get_polar_degrees(&self) -> (T, T) {
        let radius = self.length();
        let angle = self.angle_from_x_axis_positive();
        (radius, angle)
    }
    pub fn translate_vector(&mut self, vector: &Self) {
        self.translate(vector.x, vector.y);
    }
    pub fn translate_vector2d<U: Into<T> + Copy + Clone>(&mut self, vector: &Vector2d<U>) {
        self.translate(vector.x.into(), vector.y.into());
    }
    pub fn translate_vector2d_ref<U: Into<T> + Copy + Clone>(&mut self, vector: &Vector2d<U>) {
        self.translate(vector.x.into(), vector.y.into());
    }
    pub fn translate_vector2d_ref_mut<U: Into<T> + Copy + Clone>(&mut self, vector: &mut Vector2d<U>) {
        self.translate(vector.x.into(), vector.y.into());
    }
}
impl<T: Neg<Output = T> + Copy + num_traits::Zero> Vector2d<T> {
    pub fn perpendicular(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
    pub fn perpendicular_neg(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }
    pub fn perpendicular_x(&self) -> Self {
        Self {
            x: self.y,
            y: T::zero(),
        }
    }
    pub fn perpendicular_y(&self) -> Self {
        Self {
            x: T::zero(),
            y: -self.x,
        }
    }
    pub fn perpendicular_neg_x(&self) -> Self {
        Self {
            x: -self.y,
            y: T::zero(),
        }
    }
    pub fn perpendicular_neg_y(&self) -> Self {
        Self {
            x: T::zero(),
            y: self.x,
        }
    }

    pub fn negate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
    pub fn negate_x(&self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
        }
    }
    pub fn negate_y(&self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
        }
    }
    pub fn negate_both(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl<T: Zero + PartialEq + Copy + Clone> Vector2d<T> {
    #[inline]
    pub fn origin() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
    #[inline]
    pub fn is_origin(&self) -> bool {
        self.x == T::zero() && self.y == T::zero()
    }
    #[inline]
    pub fn set_origin(&mut self) {
        self.x = T::zero();
        self.y = T::zero();
    }
    #[inline]
    pub fn swap_xy(&mut self) {
        let temp = self.x;
        self.x = self.y;
        self.y = temp;
    }
    #[inline]
    pub fn splat(&mut self, value: T) {
        self.x = value;
        self.y = value;
    }
    #[inline]
    pub fn to_tuple(&self) -> (T, T) {
        (self.x, self.y)
    }
    #[inline]
    pub fn from_tuple(tuple: (T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
    #[inline]
    pub fn to_array(&self) -> [T; 2] {
        [self.x, self.y]
    }
    #[inline]
    pub fn from_array(array: [T; 2]) -> Self {
        Self {
            x: array[0],
            y: array[1],
        }
    }
}
impl<T: PartialEq> Vector2d<T> {
    pub fn is_equal(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
    pub fn is_not_equal(&self, other: &Self) -> bool {
        !self.is_equal(other)
    }
}
impl<T> From<Vector2d<T>> for (T, T)
where T: Copy + Clone,
{
    fn from(vector: Vector2d<T>) -> Self {
        (vector.x, vector.y)
    }
}
impl<T> Into<Vector2d<T>> for (T, T)
where T: Copy + Clone,
{
    fn into(self) -> Vector2d<T> {
        Vector2d {
            x: self.0,
            y: self.1,
        }
    }
}
impl<T> From<[T; 2]> for Vector2d<T>
where T: Float,
{
    fn from(array: [T; 2]) -> Self {
        Vector2d {
            x: array[0],
            y: array[1],
        }
    }
}
impl<T> Into<[T; 2]> for Vector2d<T>
where T: Float + PartialEq,
{
    fn into(self) -> [T; 2] {
        [self.x, self.y]
    }
}
impl<T: std::fmt::Display + std::fmt::Debug> std::fmt::Display for Vector2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Customize so only `x' and `y` are denoted.
        // write!(f, "x: {}, y: {}", &self.x, &self.y)
        f.debug_struct("Vector")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
impl<T: std::fmt::Debug> std::fmt::Debug for Vector2d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector2d")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
/*
/// Trait for converting a Vector2d of any numeric type to a Vector2d of a float type.
pub trait ToFloatVector2d<F: Float> {
    /// Converts the vector to a Vector2d with float components.
    fn to_float(&self) -> Vector2d<F>;
}
*/

impl<T: NumCast + Copy, F: Float> ToFloatVector2d<F> for Vector2d<T> {
    fn to_float(&self) -> Vector2d<F> {
        Vector2d {
            x: NumCast::from(self.x).unwrap_or(F::zero()),
            y: NumCast::from(self.y).unwrap_or(F::zero()),
        }
    }
}
impl<T: NumCast + Copy, F: Float> ToFloatVector2d<F> for &Vector2d<T> {
    fn to_float(&self) -> Vector2d<F> {
        Vector2d {
            x: NumCast::from(self.x).unwrap_or(F::zero()),
            y: NumCast::from(self.y).unwrap_or(F::zero()),
        }
    }
}
impl<T: NumCast + Copy, F: Float> ToFloatVector2d<F> for &mut Vector2d<T> {
    fn to_float(&self) -> Vector2d<F> {
        Vector2d {
            x: NumCast::from(self.x).unwrap_or(F::zero()),
            y: NumCast::from(self.y).unwrap_or(F::zero()),
        }
    }
}
impl<T: PartialEq> Eq for Vector2d<T> {}

impl<T: PartialOrd> PartialOrd for Vector2d<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let x_cmp = self.x.partial_cmp(&other.x)?;
        let y_cmp = self.y.partial_cmp(&other.y)?;
        if x_cmp == Ordering::Equal {
            Some(y_cmp)
        } else {
            Some(x_cmp)
        }
    }
}
impl<T: Ord> Ord for Vector2d<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let x_cmp = self.x.cmp(&other.x);
        if x_cmp == Ordering::Equal {
            self.y.cmp(&other.y)
        } else {
            x_cmp
        }
    }
}

impl<T: Hash> Hash for Vector2d<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
