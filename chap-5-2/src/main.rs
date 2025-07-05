use {
    std::{
        fs::File,
        io::{BufWriter, Write},
    },
    utils::ray::Ray,
    utils::vec3::Vec3,
};
fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = *r.origin() - *center;
    let a = r.direction().dot(r.direction());
    let b = 2. * oc.dot(r.direction());
    let c = oc.dot(&oc) - radius * radius;
    let d = b * b - 4. * a * c;

    if d <= 0. {
        -1.
    } else {
        (-b - d.sqrt()) / (2. * a)
    }
}
fn color(r: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0., 0., -1.), 0.5, r);
    if t > 0. {
        let n = (r.point_at_parametr(t) - Vec3::new(0., 0., -1.)).make_unit_vector();
        return 0.5 * Vec3::new(n.x() + 1., n.y() + 1., n.z() + 1.);
    }

    let unit_direction = r.direction().make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
}
fn render(buf_writer: &mut BufWriter<File>) -> Result<(), String> {
    const IMAGE_WIDTH: usize = 200;
    const IMAGE_HEIGHT: usize = 100;
    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::default();
    if let Err(e) = write!(buf_writer, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n") {
        return Err(format!("Ошибка записи в файл: {e:?}"));
    }
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / IMAGE_WIDTH as f64;
            let v = j as f64 / IMAGE_HEIGHT as f64;
            // первый вектор остается в начале координат, второй - пробигает все точки на плоскости
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r);

            if let Err(e) = writeln!(
                buf_writer,
                "{} {} {}",
                (255.99 * col.r()) as u8,
                (255.99 * col.g()) as u8,
                (255.99 * col.b()) as u8
            ) {
                return Err(format!("Ошибка записи в файл: {e:?}"));
            }
        }
    }

    Ok(())
}

fn main() {
    let f = match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("chap-5-2.ppm")
    {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Ошибка открытия файла: {e:?}");
            std::process::exit(1);
        }
    };
    let mut buf_writer = BufWriter::new(f);
    if let Err(e) = render(&mut buf_writer) {
        eprintln!("{e}");
        std::process::exit(1);
    };

    let _ = buf_writer.flush();
}
