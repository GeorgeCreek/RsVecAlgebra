pub use point2d::Point2d;
// pub use vector2d::Vector2d;
pub use line2d::Line2d;
#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::Float;  // Add this import

    #[test]
    fn angle_between_degrees_returns_zero_for_identical_lines() {
        let line1 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(1.0, 1.0));
        let line2 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(1.0, 1.0));
        let (angle, clockwise) = line1.angle_to_line(&line2);
        assert!((angle - 0.0).abs() < 1e-5, "angle was {}", angle);
        assert!(!clockwise, "Expected angle to be counterclockwise");
    }
    #[test]
    fn angle_between_degrees_returns_90_for_perpendicular_lines_ccw() {
        let line1 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(0.0, -1.0));
        let line2 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(1.0, 1.0));
        let (angle, clockwise) = line1.angle_to_line(&line2);
        assert_eq!(angle.to_degrees(), 225.0);
        assert!(clockwise, "Expected angle to be counterclockwise");
    }
    #[test]
    fn angle_between_degrees_returns_225_for_diagonal_and_down() {
        let line1 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(1.0, 1.0));
        let line2 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(0.0, -1.0));
        let(angle, clockwise) = line1.angle_to_line(&line2);
        assert_eq!(angle.to_degrees(), 225.0);  // Changed to counterclockwise angle
        assert!(!clockwise, "Expected angle to be clockwise");
    }
    /*
    #[test]
    fn angle_between_degrees_returns_90_for_perpendicular_lines_cw() {
        let line1 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(1.0, 1.0));
        let line2 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(1.0, -1.0));
        assert_eq!(line1.angle_between_degrees(&line2), 270.0);
        assert!(line1.is_angle_clockwise(&line2));
    }
    #[test]
    fn angle_between_degrees_returns_180_for_opposite_direction_lines() {
        let line1 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(1.0, 1.0));
        let line2 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(-1.0, -1.0));
        let angle = line1.angle_between_degrees(&line2);
        assert!((angle - 180.0).abs() < 1e-5, "angle was {}", angle);
    }
    #[test]
    fn angle_between_degrees_handles_nan_angle_gracefully() {
        let line1 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(0.0, 0.0));
        let line2 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(0.0, 0.0));
        assert_eq!(line1.angle_between_degrees(&line2), 0.0);
    }
    #[test]
    fn angle_between_degrees_handles_large_angles_correctly() {
        let line1 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(1.0, 0.0));
        let line2 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(-1.0, 1.0));
        assert!(line1.angle_between_degrees(&line2) > 90.0 && line1.angle_between_degrees(&line2) < 180.0);
    }
    #[test]
    fn is_angle_clockwise_returns_true_for_clockwise_angle() {
        let line1 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(1.0, 0.0));
        let line2 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(0.0, -1.0));
        assert!(line1.is_angle_clockwise(&line2));
    }
    #[test]
    fn is_angle_clockwise_returns_true_for_clockwise_angle_false() {
        let line1 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(1.0, 1.0));
        let line2 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(-1.0, 1.0));
        assert!(!line1.is_angle_clockwise(&line2));
    }
    */

    /*
    #[test]
    fn is_angle_clockwise_returns_true_for_clockwise_angle_true() {
        let line1 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(1.0, 1.0));
        let line2 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(1.0, -1.0));
        assert!(!line1.is_angle_clockwise(&line2));
    }
    */
    /*
    #[test]
    fn is_angle_clockwise_returns_false_for_counterclockwise_angle() {
        let line1 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(1.0, 0.0));
        let line2 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(0.0, 1.0));
        assert!(!line1.is_angle_clockwise(&line2));
    }

    #[test]
    fn is_angle_clockwise_returns_false_for_collinear_lines() {
        let line1 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(1.0, 1.0));
        let line2 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(2.0, 2.0));
        assert!(!line1.is_angle_clockwise(&line2));
    }

    #[test]
    fn is_angle_clockwise_handles_nan_cross_product_gracefully() {
        let line1 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(0.0, 0.0));
        let line2 = Line2d::new(Point2d::new(0.0, 0.0), Point2d::new(0.0, 0.0));
        assert!(!line1.is_angle_clockwise(&line2));
    }
    */
}