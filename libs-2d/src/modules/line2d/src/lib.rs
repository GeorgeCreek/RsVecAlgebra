use point2d::Point2d;
use num_traits::{Float, Zero};
use vector2d::Vector2d;

pub struct Line2d<T> {
    pub pt1: Point2d<T>,
    pub pt2: Point2d<T>,
}
impl<T: Copy + Clone + Zero + Float> Line2d<T> {
    pub fn new(pt1: Point2d<T>, pt2: Point2d<T>) -> Self {
        Line2d { pt1, pt2 }
    }

    pub fn length(&self) -> T {
        if self.pt1 == self.pt2 {
            return T::zero(); // If both points are the same, length is zero
        }
        // Calculate the length using the distance formula
        let length = (self.length_squared()).sqrt();
        if length.is_nan() || length.is_infinite() {
            return T::zero(); // Return zero if length is NaN or infinite
        }
        let epsilon = T::from(1e-10).unwrap_or(T::zero());
        let neg_epsilon = T::from(-1e-10).unwrap_or(T::zero());
        if length < epsilon && length > neg_epsilon {
            return T::zero();
        }
        if length < T::zero() {
            return T::zero(); // Return zero if length is negative
        }
        length
    }
    pub fn set_length(&mut self, length: T) {
        if length.is_nan() || length.is_infinite() || length < T::zero() {
            return; // Do not change the line if length is NaN, infinite, or negative
        }
        let unit_vector = self.unit_vector();

        // Create a new point using vector components
        let dx = unit_vector.dx() * length;
        let dy = unit_vector.dy() * length;
        self.pt2 = Point2d::new(self.pt1.x() + dx, self.pt1.y() + dy);
    }
    pub fn angle(&self) -> T {
        if self.is_point() {
            return T::zero(); // If the line is a point, angle is zero
        }
        if self.dx().is_nan() || self.dy().is_nan() {
            return T::zero(); // Return zero if dx or dy is NaN
        }
        if self.dx().is_infinite() || self.dy().is_infinite() {
            return T::zero(); // Return zero if dx or dy is infinite
        }
        let mut angle = self.dy().atan2(self.dx()); // Calculate the angle in radians
        if angle.is_nan() || angle.is_infinite() {
            return T::zero(); // Return zero if angle is NaN or infinite
        }
        let epsilon = T::from(1e-10).unwrap_or(T::zero());
        let neg_epsilon = T::from(-1e-10).unwrap_or(T::zero());
        if angle < epsilon && angle > neg_epsilon {
            return T::zero(); // Return zero if angle is close to zero
        }
        if angle < T::zero() {
            angle = angle + T::from(2.0 * std::f64::consts::PI).unwrap_or(T::zero()); // Normalize to [0, 2π)
        }
        if angle > T::from(2.0 * std::f64::consts::PI).unwrap_or(T::zero()) {
            angle = angle - T::from(2.0 * std::f64::consts::PI).unwrap_or(T::zero()); // Normalize to [0, 2π)
        }
        angle
    }
    pub fn angle_degrees(&self) -> T {
        let angle = self.angle();
        if angle.is_nan() || angle.is_infinite() {
            return T::zero(); // Return zero if angle is NaN or infinite
        }
        let degrees = angle.to_degrees();
        if degrees.is_nan() || degrees.is_infinite() {
            return T::zero(); // Return zero if degrees is NaN or infinite
        }
        let epsilon = T::from(1e-10).unwrap_or(T::zero());
        let neg_epsilon = T::from(-1e-10).unwrap_or(T::zero());
        if degrees < epsilon && degrees > neg_epsilon {
            return T::zero(); // Return zero if degrees is close to zero
        }
        degrees
    }
    pub fn set_angle(&mut self, angle: T) {
        if angle.is_nan() || angle.is_infinite() {
            return; // Do not change the line if angle is NaN or infinite
        }
        let length = self.length();
        if length.is_nan() || length.is_infinite() || length < T::zero() {
            return; // Do not change the line if length is NaN, infinite, or negative
        }
        let dx = length * angle.cos();
        let dy = length * angle.sin();
        self.pt2.set_x(self.pt1.x() + dx);
        self.pt2.set_y(self.pt1.y() + dy);
    }
    pub fn set_angle_degrees(&mut self, angle_degrees: T) {
        if angle_degrees.is_nan() || angle_degrees.is_infinite() {
            return; // Do not change the line if angle is NaN or infinite
        }
        let angle = angle_degrees.to_radians();
        self.set_angle(angle);
    }
    pub fn dot(&self, other: &Line2d<T>) -> T {
        let dx1 = self.dx();
        let dy1 = self.dy();
        let dx2 = other.dx();
        let dy2 = other.dy();
        dx1 * dx2 + dy1 * dy2
    }
    pub fn cross(&self, other: &Line2d<T>) -> T {
        let dx1 = self.dx();
        let dy1 = self.dy();
        let dx2 = other.dx();
        let dy2 = other.dy();
        dx1 * dy2 - dy1 * dx2
    }
    pub fn get_polar(&self) -> (T, T) {
        let dx = self.dx();
        let dy = self.dy();
        let length = (dx * dx + dy * dy).sqrt();
        if length.is_nan() || length.is_infinite() {
            return (T::zero(), T::zero()); // Return zero if length is NaN
        }
        let mut angle = dy.atan2(dx); // Calculate the angle in radians
        if angle.is_nan() || angle.is_infinite() {
            return (T::zero(), T::zero()); // Return zero if angle is NaN
        }
        let epsilon = T::from(1e-10).unwrap_or(T::zero());
        let neg_epsilon = T::from(-1e-10).unwrap_or(T::zero());
        if length < epsilon && length > neg_epsilon {
            return (T::zero(), T::zero()); // Return zero if length is close to zero
        }
        if angle < epsilon && angle > neg_epsilon {
            return (T::zero(), T::zero()); // Return zero if angle is close to zero
        }
        if angle < T::zero() {
            angle = angle + T::from(2.0 * std::f64::consts::PI).unwrap_or(T::zero()); // Normalize to [0, 2π)
        }
        if angle > T::from(2.0 * std::f64::consts::PI).unwrap_or(T::zero()) {
            angle = angle - T::from(2.0 * std::f64::consts::PI).unwrap_or(T::zero()); // Normalize to [0, 2π)
        }
        (length, angle)
    }
    pub fn set_polar(&self, length: T, angle: T) -> (T, T) {
        if length.is_nan() || length.is_infinite() || length < T::zero() {
            return (T::zero(), T::zero()); // Return zero if length is NaN, infinite, or negative
        }
        let dx = length * angle.cos();
        let dy = length * angle.sin();
        if dx.is_nan() || dy.is_nan() {
            return (T::zero(), T::zero()); // Return zero if dx or dy is NaN
        }
        if dx.is_infinite() || dy.is_infinite() {
            return (T::zero(), T::zero()); // Return zero if dx or dy is infinite
        }
        (self.pt1.x() + dx, self.pt1.y() + dy)
    }
    pub fn scale(&mut self, scale: T) {
        if scale.is_nan() || scale.is_infinite() || scale < T::zero() {
            return; // Do not change the line if scale is NaN, infinite, or negative
        }
        if scale == T::zero() {
            self.pt2.set_x(self.pt1.x());
            self.pt2.set_y(self.pt1.y());
            return; // If scale is zero, set pt2 to pt1
        }
        // Scale the line by the given factor
        self.pt2.set_x(self.pt1.x() + self.dx() * scale);
        self.pt2.set_y(self.pt1.y() + self.dy() * scale);
    }
    pub fn length_squared(&self) -> T {
        self.dx() * self.dx() + self.dy() * self.dy()
    }
    pub fn point_at(&self, t: T) -> Point2d<T> {
        if t < T::zero() || t > T::one() {
            return Point2d::new(T::zero(), T::zero()); // Return zero point if t is out of bounds
        }
        let x = self.pt1.x() + self.dx() * t;
        let y = self.pt1.y() + self.dy() * t;
        Point2d::new(x, y)
    }
    pub fn midpoint(&self) -> Point2d<T> {
        Point2d::new(
            (self.pt1.x() + self.pt2.x()) / T::from(2).unwrap(),
            (self.pt1.y() + self.pt2.y()) / T::from(2).unwrap(),
        )
    }
}
impl<T> Line2d<T>
where T: Copy + Clone + Zero + Float,
{
    pub fn pt1(&self) -> &Point2d<T> {
        &self.pt1
    }
    pub fn x1(&self) -> T {
        self.pt1.x()
    }
    pub fn set_x1(&mut self, x: T) {
        self.pt1.set_x(x);
    }
    pub fn y1(&self) -> T {
        self.pt1.y()
    }
    pub fn pt2(&self) -> &Point2d<T> { &self.pt2 }
    pub fn x2(&self) -> T {
        self.pt2.x()
    }
    pub fn y2(&self) -> T {
        self.pt2.y()
    }
    pub fn set_pt1(&mut self, pt: Point2d<T>) {
        self.pt1 = pt;
    }
    pub fn set_pt2(&mut self, pt: Point2d<T>) {
        self.pt2 = pt;
    }
    pub fn set_pt1_coordinates(&mut self, x: T, y: T) {
        self.pt1 = Point2d::new(x, y);
    }
    pub fn set_pt2_coordinates(&mut self, x: T, y: T) {
        self.pt2 = Point2d::new(x, y);
    }
    pub fn set_line_from_points(&mut self, pt1: Point2d<T>, pt2: Point2d<T>) {
        self.pt1 = pt1;
        self.pt2 = pt2;
    }
    pub fn set_line_from_coordinates(&mut self, x1: T, y1: T, x2: T, y2: T) {
        self.pt1 = Point2d::new(x1, y1);
        self.pt2 = Point2d::new(x2, y2);
    }
    pub fn dx(&self) -> T {
        self.pt2.x() - self.pt1.x()
    }
    pub fn dy(&self) -> T {
        self.pt2.y() - self.pt1.y()
    }
    pub fn delta_xy(&self) -> Point2d<T> {
        Point2d::new(self.dx(), self.dy())
    }
    pub fn slope(&self) -> T {
        if self.dx() == T::zero() {
            T::infinity() // Vertical line
        } else {
            self.dy() / self.dx()
        }
    }
    pub fn is_vertical(&self) -> bool {
        self.dx() == T::zero()
    }
    pub fn is_horizontal(&self) -> bool {
        self.dy() == T::zero()
    }
    pub fn is_point(&self) -> bool {
        self.pt1 == self.pt2
    }
    pub fn normal(&self) -> Line2d<T> {
        let dx = self.dx();
        let dy = self.dy();
        Line2d::new(
            Point2d::new(self.pt1.x() - dy, self.pt1.y() + dx),
            Point2d::new(self.pt2.x() - dy, self.pt2.y() + dx),
        )
    }
    pub fn normal_vector(&self) -> Vector2d<T> {
        Vector2d::new(-self.dy(), self.dx())
    }
    pub fn to_vector(&self) -> Vector2d<T> {
        Vector2d::new(self.dx(), self.dy())
    }
    pub fn unit_vector(&self) -> Line2d<T> {
        let length = self.length();
        if length.is_nan() || length.is_infinite() {
            return Line2d::new(self.pt1, self.pt2); // Return the same line if NaN or infinite
        }
        if length == T::zero() {
            return Line2d::new(self.pt1, self.pt2); // Return the same line if length is zero
        }
        let mut unit_dx = self.dx() / length;
        let mut unit_dy = self.dy() / length;
        if unit_dx.is_nan() || unit_dy.is_nan() {
            return Line2d::new(self.pt1, self.pt2); // Return the same line if NaN
        }
        if unit_dx.is_infinite() || unit_dy.is_infinite() {
            return Line2d::new(self.pt1, self.pt2); // Return the same line if infinite
        }
        if unit_dx < T::from(1e-10).unwrap_or(T::zero()) && unit_dx > T::from(-1e-10).unwrap_or(T::zero()) {
            unit_dx = T::zero();
        }
        if unit_dy < T::from(1e-10).unwrap_or(T::zero()) && unit_dy > T::from(-1e-10).unwrap_or(T::zero()) {
            unit_dy = T::zero();
        }
        if unit_dx.is_zero() && unit_dy.is_zero() {
            return Line2d::new(self.pt1, self.pt2); // Return the same line if both components are zero}
        }
        Line2d::new(
            self.pt1,
            Point2d::new(self.pt1.x() + unit_dx, self.pt1.y() + unit_dy),
        )
    }
    pub fn perpendicular_bisector(&self) -> Line2d<T> {
        let midpoint = self.midpoint();
        let normal_vector = self.normal_vector();
        Line2d::new(
            midpoint,
            Point2d::new(midpoint.x() + normal_vector.x, midpoint.y() + normal_vector.y),
        )
    }
    pub fn is_collinear(&self, other: &Line2d<T>) -> bool {
        let cross_product = self.cross(&other);
        cross_product.is_zero()
    }
    pub fn is_parallel(&self, other: &Line2d<T>) -> bool {
        let cross_product = self.cross(&other);
        cross_product.is_zero()
    }
    pub fn is_perpendicular(&self, other: &Line2d<T>) -> bool {
        let dot_product = self.dot(&other);
        dot_product.is_zero()
    }
    pub fn is_orthogonal(&self, other: &Line2d<T>) -> bool {
        let dot_product = self.dot(&other);
        dot_product.is_zero()
    }
    pub fn is_same_direction(&self, other: &Line2d<T>) -> bool {
        let dot_product = self.dot(&other);
        dot_product > T::zero()
    }
    pub fn is_opposite_direction(&self, other: &Line2d<T>) -> bool {
        let dot_product = self.dot(&other);
        dot_product < T::zero()
    }
    pub fn angle_between_dot(&self, other: &Line2d<T>) -> T {
        let dot_product = self.dot(&other);
        let length_self = self.length();
        let length_other = other.length();
        if length_self.is_zero() || length_other.is_zero() {
            return T::zero(); // Return zero if either line is a point
        }
        let cos_theta = dot_product / (length_self * length_other);
        if cos_theta.is_nan() || cos_theta.is_infinite() {
            return T::zero(); // Return zero if cos_theta is NaN or infinite
        }
        cos_theta.acos()
    }
    pub fn is_angle_clockwise(&self, other: &Line2d<T>) -> bool {
        let cross_product = self.cross(&other);
        if cross_product.is_nan() || cross_product.is_infinite() {
            return false; // Return false if cross product is NaN or infinite
        }
        cross_product < T::zero() // Clockwise if cross product is negative
    }
    pub fn angle_between_deg(&self, other: &Line2d<T>) -> T {
        let angle = self.angle_between_dot(other);
        if angle.is_nan() || angle.is_infinite() {
            return T::zero(); // Return zero if angle is NaN or infinite
        }
        let degrees = angle.to_degrees();
        if degrees.is_nan() || degrees.is_infinite() {
            return T::zero(); // Return zero if degrees is NaN or infinite
        }
        let epsilon = T::from(1e-10).unwrap_or(T::zero());
        let neg_epsilon = T::from(-1e-10).unwrap_or(T::zero());
        if degrees < epsilon && degrees > neg_epsilon {
            return T::zero(); // Return zero if degrees is close to zero
        }
        degrees
    }
    pub fn angle_to_line(&self, other: &Line2d<T>) -> (T, bool) {
        let angle_self = self.angle();
        let angle_other = other.angle();
        // Calculate the difference between angles
        let angle_diff = angle_other - angle_self;
        if angle_diff.is_nan() || angle_diff.is_infinite() {
            return (T::zero(), false) // Return zero if angle difference is NaN or infinite
        }
        let is_clockwise:bool = angle_diff < T::zero();
        (angle_diff.abs(), is_clockwise)
    }
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
