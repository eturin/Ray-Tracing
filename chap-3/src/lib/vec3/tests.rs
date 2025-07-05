use super::*;
#[test]
fn test_base() {
    let a = Vec3::default();
    assert_eq!(a.x(), 0.);
    assert_eq!(a.y(), 0.);
    assert_eq!(a.z(), 0.);

    let a = Vec3::new(1., 2., 3.);
    assert_eq!(a.x(), 1.);
    assert_eq!(a.y(), 2.);
    assert_eq!(a.z(), 3.);

    assert_eq!(a.deref(), &[1., 2., 3.]);
    assert_eq!(&*a, &[1., 2., 3.]);

    let mut a = Vec3::new(1., 2., 3.);
    assert_eq!(&mut *a, &mut [1., 2., 3.]);
}

#[test]
fn test_ops() {
    let a = Vec3::new(1., 2., 3.);
    let b = Vec3::new(2., 3., 4.);
    let c = a + b;
    assert_eq!(c.x(), 3.);
    assert_eq!(c.y(), 5.);
    assert_eq!(c.z(), 7.);

    let a = Vec3::new(1., 2., 3.);
    let b = Vec3::new(2., 3., 4.);
    let c = a - b;
    assert_eq!(c.x(), -1.);
    assert_eq!(c.y(), -1.);
    assert_eq!(c.z(), -1.);

    let a = Vec3::new(1., 2., 3.);
    let b = Vec3::new(2., 3., 4.);
    let c = a * b;
    assert_eq!(c.x(), 2.);
    assert_eq!(c.y(), 6.);
    assert_eq!(c.z(), 12.);

    let mut a = Vec3::new(1., 2., 3.);
    let b = Vec3::new(2., 3., 4.);
    a += b;
    assert_eq!(a.x(), 3.);
    assert_eq!(a.y(), 5.);
    assert_eq!(a.z(), 7.);

    let mut a = Vec3::new(1., 2., 3.);
    let b = Vec3::new(2., 3., 4.);
    a -= b;
    assert_eq!(a.x(), -1.);
    assert_eq!(a.y(), -1.);
    assert_eq!(a.z(), -1.);

    let mut a = Vec3::new(1., 2., 3.);
    let b = Vec3::new(2., 3., 4.);
    a *= b;
    assert_eq!(a.x(), 2.);
    assert_eq!(a.y(), 6.);
    assert_eq!(a.z(), 12.);

    let a = Vec3::new(2., 6., 12.);
    let b = Vec3::new(2., 3., 4.);
    let c = a / b;
    assert_eq!(c.x(), 1.);
    assert_eq!(c.y(), 2.);
    assert_eq!(c.z(), 3.);

    let mut a = Vec3::new(2., 6., 12.);
    let b = Vec3::new(2., 3., 4.);
    a /= b;
    assert_eq!(a.x(), 1.);
    assert_eq!(a.y(), 2.);
    assert_eq!(a.z(), 3.);
    a *= 2.;
    assert_eq!(a.x(), 2.);
    assert_eq!(a.y(), 4.);
    assert_eq!(a.z(), 6.);
    a /= 2.;
    assert_eq!(a.x(), 1.);
    assert_eq!(a.y(), 2.);
    assert_eq!(a.z(), 3.);

    let a = Vec3::new(2., 6., 12.);
    let c = a / 2.;
    assert_eq!(c.x(), 1.);
    assert_eq!(c.y(), 3.);
    assert_eq!(c.z(), 6.);

    let a = Vec3::new(1., 2., 3.);
    let b = a * 2.;
    assert_eq!(b.x(), 2.);
    assert_eq!(b.y(), 4.);
    assert_eq!(b.z(), 6.);

    let a = Vec3::new(1., 2., 3.);
    let b = 2. * a;
    assert_eq!(b.x(), 2.);
    assert_eq!(b.y(), 4.);
    assert_eq!(b.z(), 6.);

    let x = b[0];
    assert_eq!(x, 2.);

    let mut a = Vec3::new(1., 2., 3.);
    a[0] = 10.;
    let x = a[0];
    assert_eq!(x, 10.);

    let b = -a;
    assert_eq!(b[0], -10.);
}

#[test]
fn test_dot_cross() {
    let a = Vec3::new(1., 2., 3.);
    let b = Vec3::new(2., 3., 4.);
    assert_eq!(a.dot(&b), 1. * 2. + 2. * 3. + 3. * 4.);

    let c = a.cross(&b);
    assert_eq!(c.x(), 2. * 4. - 3. * 3.);
    assert_eq!(c.y(), 0. - 1. * 4. + 2. * 3.);
    assert_eq!(c.z(), 1. * 3. - 2. * 2.);
}

#[test]
fn test_len() {
    let a = Vec3::new(1., 2., 3.);
    assert_eq!(a.length_squared(), 1. * 1. + 2. * 2. + 3. * 3.);

    let a = 5. * Vec3::new(0., 4., 3.);
    let e = a.make_unit_vector();
    assert_eq!(e.len(), 1.);

    let a = 5. * Vec3::new(0., 0., 0.);
    let e = a.make_unit_vector();
    assert_eq!(e.len(), 0.);
}
