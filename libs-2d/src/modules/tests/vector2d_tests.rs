pub use vector2d::Vector2d;
// use line2d::Line2D;
#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::identities::Zero;
    use std::ops::{Add, Mul};

    #[test]
    fn creates_vector_with_positive_components() {
        let vector = Vector2d::new(10, 20);
        assert_eq!(vector.x, 10);
        assert_eq!(vector.y, 20);
    }

    #[test]
    fn creates_vector_with_negative_components() {
        let vector = Vector2d::new(-15, -25);
        assert_eq!(vector.x, -15);
        assert_eq!(vector.y, -25);
    }

    #[test]
    fn creates_vector_with_mixed_components() {
        let vector = Vector2d::new(-30, 40);
        assert_eq!(vector.x, -30);
        assert_eq!(vector.y, 40);
    }

    #[test]
    fn creates_null_vector_with_zero_trait() {
        let vector: Vector2d<i32> = Vector2d::null();
        assert_eq!(vector.x, 0);
        assert_eq!(vector.y, 0);
    }

    #[test]
    fn creates_null_vector_with_custom_zero_trait() {
        #[derive(Copy, Clone, PartialEq, PartialOrd, Debug)] // Added PartialOrd
        struct CustomZeroType(i32);

        // Implement Add trait for CustomZeroType
        impl Add for CustomZeroType {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                CustomZeroType(self.0 + other.0)
            }
        }

        // Implement Mul trait for CustomZeroType
        impl Mul for CustomZeroType {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                CustomZeroType(self.0 * other.0)
            }
        }

        impl Zero for CustomZeroType {
            fn zero() -> Self { CustomZeroType(0) }
            fn is_zero(&self) -> bool { self.0 == 0 }
        }

        let vector = Vector2d::<CustomZeroType>::null();
        assert_eq!(vector.x, CustomZeroType(0));
        assert_eq!(vector.y, CustomZeroType(0));
    }



    #[test]
    fn creates_vector_with_large_positive_components() {
        let vector = Vector2d::new(i32::MAX, i32::MAX);
        assert_eq!(vector.x, i32::MAX);
        assert_eq!(vector.y, i32::MAX);
    }

    #[test]
    fn creates_vector_with_large_negative_components() {
        let vector = Vector2d::new(i32::MIN, i32::MIN);
        assert_eq!(vector.x, i32::MIN);
        assert_eq!(vector.y, i32::MIN);
    }

    #[test]
    fn creates_vector_with_zero_components() {
        let vector = Vector2d::new(0, 0);
        assert_eq!(vector.x, 0);
        assert_eq!(vector.y, 0);
    }
}
