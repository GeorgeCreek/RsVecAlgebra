// use libs_2d::modules;
use line2d::Line2D;

// extern crate libs_2d;
use point2d::Point2D;

#[test]
fn test_angle_from_xaxis_pos() {
    let mut clockwise: bool;
    let angle1 = Point2D::new(6.0, -8.0).angle_from_x_axis_pos();
    ///println!("Angle: {}", angle1);
    let angle2 = Point2D::new(3.0, 1.0).angle_from_x_axis_pos();
    //println!("Angle: {}", angle2);
    clockwise = true;
    if angle1 > angle2 {
        clockwise = false;
    }
    println!("ANGLE1: {} ANGLE2: {} CW: {}", angle1, angle2, clockwise);
}
fn main() {
    let point1 = Point2D::new(1.0, 2.0);
    println!("Point {}", point1);
    let line = Line2D::new(Point2D::new(2.0, 3.0),
                                       Point2D::new(5.0, 7.0));
    println!("Line: {}", line);
    let p1 = Point2D::new(1.5, -3.8);
    // let pm: Point2D<f32> = p1.into();
    let pm: (f32, f32) = p1.into();
    println!("PM {:?}", pm);
    let p2 = Point2D::new(3.5, -5.8);
    let pm1: [f32; 2] = p2.into();
    println!("PM {:?}", pm1);
    // assert_eq!(pm, Point2D::new(1.5, -3.8));
    // let p2: Point2D<f64> = Point2D::from(pm);
    // let pt = Point2D::origin();
    // println!("PT {}", pt);
    // assert!(pt.equal(Point2D::new(0.0, 0.0)));
    let mut point = Point2D::new(3.0, 4.0);
    point.set_origin();
    println!("Move to {}", point);
    let mut point = Point2D::new(9.0, 16.0);
    point = point.sqrt();
    println!("Sqrt: {}", point);
    point = point.pow2();
    println!("Pow2: {}", point);
    let mut point = Point2D::new(-3.0, -4.0);
    point = point.abs();
    println!("Abs: {}", point);
    let sqr_magnitude = point.sqr_magnitude();
    println!("Square Magnitude: {}", sqr_magnitude);
    let magnitude = point.magnitude();
    println!("Magnitude: {}", magnitude);
    let mut point = Point2D::new(5.0, 5.0);
    let normalized = point.normalized();
    println!("Point: {} to Normalized: {}", point, normalized);
    point.normalize();
    println!("Point: {} to Normalized: {}", point, point);
    let mut point = Point2D::new(3.0, 4.0);
    let clamp_magnitude = point.clamp_magnitude(0.75);
    println!("Clamp Magnitude: {}", clamp_magnitude);
    let point = Point2D::new(3.0, 4.0);
    let length = point.length();
    println!("Length: {}", length);
    let angle = point.angle();
    println!("Angle Rad: {}", angle);
    let p1 = Point2D::new(3.0, 4.0);
    let p2 = Point2D::new(-3.0, 4.0);
    println!("Dot: {}", p1.dot(&p2));
    println!("Angle to Rad: {}", p1.angle_to(&p2));
    let point = Point2D::new(3.0, -4.0);
    let manhattan_magnitude = point.manhattan_magnitude();
    println!("manhattan magnitude: {}", manhattan_magnitude);
    let p1 = Point2D::new(3.0, 4.0);
    let p2 = Point2D::new(-3.0, 4.0);
    let manhattan_distance = p1.manhattan_distance(p2);
    println!("manhattan distance: {}", manhattan_distance);
    let midpoint_to = p1.midpoint_to(&p2);
    println!("Midpoint To: {}", midpoint_to);
    let mut point = Point2D::new(-3.0, 4.0);
    point.translate(6.0, -8.0);
    println!("Translate To: {}", point);
    let distance_from_origin = point.dist_from_origin();
    println!("Distance From Origin: {}", distance_from_origin);
    let p1 = Point2D::new(3.0, 4.0);
    let p2 = Point2D::new(-3.0, 4.0);
    let lerp = p1.lerp(p2, 0.75);
    println!("LERP: {}", lerp);
    let point = Point2D::new(3.0, 4.0);
    let opposite = point.set_neg();
    println!("Opposite: {}", opposite);
    let mut point = Point2D::new(0.0, 0.0);
    point.set_polar(5.0, 45.);
    println!("Set polar: {}", point);
}
