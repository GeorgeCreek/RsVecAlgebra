mod vector2d_tests;
mod point2d_tests;
mod line2d_tests;

use vector2d::Vector2d;
/// use num_traits::{Float, Signed};
///use::num_traits::identities::Zero;


#[test]
fn creates_vector_with_given_components() {
    let vector = Vector2d::new(3.0, 4.0);
    assert_eq!(vector.x, 3.0);
    assert_eq!(vector.y, 4.0);
}

#[test]
fn creates_vector_with_zero_components() {
    let vector = Vector2d::new(0.0, 0.0);
    assert_eq!(vector.x, 0.0);
    assert_eq!(vector.y, 0.0);
}

#[test]
fn creates_vector_with_negative_components() {
    let vector = Vector2d::new(-3.0, -4.0);
    assert_eq!(vector.x, -3.0);
    assert_eq!(vector.y, -4.0);
}
#[test]
fn creates_vector_with_positive_components() {
    let vector = Vector2d::new(10, 20);
    assert_eq!(vector.x, 10);
    assert_eq!(vector.y, 20);
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
fn returns_null_vector_as_default() {
    let vector = Vector2d::<f32>::default(/* Vector2d<f32> */);
    assert_eq!(vector.x, 0.0);
    assert_eq!(vector.y, 0.0);
}
#[test]
fn default_vector_is_zero() {
    let vector = Vector2d::<f32>::default();
    assert!(vector.is_zero());
}
#[test]
fn default_vector_is_not_positive() {
    let vector = Vector2d::<f32>::default();
    assert!(!vector.is_positive());
}
#[test]
fn default_vector_is_not_negative() {
    let vector = Vector2d::<f32>::default();
    assert!(!vector.is_negative());
}
#[test]
fn identifies_null_vector_correctly() {
    let vector = Vector2d::new(0, 0);
    assert!(vector.is_null());
}
#[test]
fn identifies_non_null_vector_correctly() {
    let vector = Vector2d::new(1, 0);
    assert!(vector.is_not_null());
}
#[test]
fn calculates_square_root_of_positive_vector_components() {
    let vector = Vector2d::new(4.0_f32, 9.0_f32);
    let result = vector.sqrt();
    assert_eq!(result.x, 2.0);
    assert_eq!(result.y, 3.0);
}
#[test]
fn calculates_square_root_of_zero_vector_components() {
    let vector = Vector2d::new(0.0_f32, 0.0_f32);
    let result = vector.sqrt();
    assert_eq!(result.x, 0.0);
    assert_eq!(result.y, 0.0);
}

#[test]
fn calculates_square_root_of_negative_vector_components() {
    let vector = Vector2d::new(-4.0_f32, -9.0_f32);
    let result = vector.sqrt();
    assert!(result.x.is_nan());
    assert!(result.y.is_nan());
}