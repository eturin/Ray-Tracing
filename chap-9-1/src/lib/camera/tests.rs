use crate::camera::Camera;

#[test]
fn test_base() {
    let a = Camera::default();
    assert_eq!(a.origin()[0], -0.);
    assert_eq!(a.lower_left_corner()[0], -2.);
    assert_eq!(a.horizontal()[0], 4.);
    assert_eq!(a.vertical()[1], 2.);
}
