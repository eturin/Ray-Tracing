use crate::metal::Metal;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

#[test]
fn test_base() {
    let a = Sphere::new(
        Vec3::new(1., 2., 3.),
        7.,
        Some(Box::new(Metal::new(Vec3::new(0.4, 0.7, 0.3), 0.1))),
    );
    assert_eq!(a.radius, 7.);
    assert_eq!(a.center[0], 1.);
}
