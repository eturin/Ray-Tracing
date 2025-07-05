use super::*;
use crate::vec3::Vec3;

#[test]
fn test_base() {
    let o = Vec3::new(1., 2., 3.);
    let d = Vec3::new(2., 3., 4.);
    let a = Ray::new(o, d);
    assert_eq!(&*a.origin, [1., 2., 3.]);
    assert_eq!(&*a.direction, [2., 3., 4.]);

    let a = Ray::default();
    assert_eq!(&*a.origin, [0., 0., 0.]);
    assert_eq!(&*a.direction, [0., 0., 0.]);
}

#[test]
fn test_at() {
    let o = Vec3::new(1., 2., 3.);
    let d = Vec3::new(2., 3., 4.);
    let a = Ray::new(o, d);
    assert_eq!(&*a.at(3.), [7., 11., 15.]);
    assert_eq!(&*a.origin, [1., 2., 3.]);
}
