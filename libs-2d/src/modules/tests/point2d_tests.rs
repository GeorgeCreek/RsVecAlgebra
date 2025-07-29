// pub use vector2d::Vector2d;
pub use point2d::Point2d;
// use line2d::Line2D;
#[cfg(test)]
mod tests {
    use super::*;
    //use num_traits::identities::Zero;
    #[test]
    fn x_returns_correct_value() {
        let point = Point2d::new(10, 20);
        assert_eq!(point.x(), 10);
    }

    #[test]
    fn y_returns_correct_value() {
        let point = Point2d::new(10, 20);
        assert_eq!(point.y(), 20);
    }

    #[test]
    fn to_vector_returns_correct_vector() {
        let point = Point2d::new(10, 20);
        let vector = point.to_vector();
        assert_eq!(vector.x, 10);
        assert_eq!(vector.y, 20);
    }
    #[test]
    fn length_returns_correct_value_for_positive_coordinates() {
        let point = Point2d::new(3.0, 4.0);
        assert_eq!(point.length(), 5.0);
    }

    #[test]
    fn length_returns_zero_for_origin() {
        let point = Point2d::new(0.0, 0.0);
        assert_eq!(point.length(), 0.0);
    }

    #[test]
    fn length_returns_correct_value_for_negative_coordinates() {
        let point = Point2d::new(-3.0, -4.0);
        assert_eq!(point.length(), 5.0);
    }

    #[test]
    fn length_handles_large_values_correctly() {
        let point = Point2d::new(1e10, 1e10);
        assert_eq!(point.length(), (2e20_f64).sqrt());
    }

    #[test]
    fn length_handles_small_values_correctly() {
        let point = Point2d::new(1e-10, 1e-10);
        assert_eq!(point.length(), (2e-20_f64).sqrt());
    }
    #[test]
    fn angle_returns_zero_for_positive_x_axis() {
        let point = Point2d::new(1.0, 0.0);
        assert_eq!(point.angle(), 0.0);
    }
    #[test]
    fn angle_returns_pi_for_negative_x_axis() {
        let point = Point2d::new(-1.0, 0.0);
        assert_eq!(point.angle(), std::f64::consts::PI);
    }
    #[test]
    fn angle_returns_half_pi_for_positive_y_axis() {
        let point = Point2d::new(0.0, 1.0);
        assert_eq!(point.angle(), std::f64::consts::FRAC_PI_2);
    }

    #[test]
    fn angle_returns_negative_half_pi_for_negative_y_axis() {
        let point = Point2d::new(0.0, -1.0);
        assert_eq!(point.angle(), -std::f64::consts::FRAC_PI_2);
    }

    #[test]
    fn angle_handles_quadrant_one_correctly() {
        let point = Point2d::new(1.0, 1.0);
        assert!(point.angle() > 0.0 && point.angle() < std::f64::consts::FRAC_PI_2);
    }

    #[test]
    fn angle_handles_quadrant_two_correctly() {
        let point = Point2d::new(-1.0, 1.0);
        assert!(point.angle() > std::f64::consts::FRAC_PI_2 && point.angle() < std::f64::consts::PI);
    }
    #[test]
    fn angle_handles_quadrant_three_correctly() {
        let point = Point2d::new(-1.0, -1.0);
        assert!(point.angle() < -std::f64::consts::FRAC_PI_2 && point.angle() > -std::f64::consts::PI);
    }
    #[test]
    fn angle_handles_quadrant_four_correctly() {
        let point = Point2d::new(1.0, -1.0);
        assert!(point.angle() < 0.0 && point.angle() > -std::f64::consts::FRAC_PI_2);
    }
}