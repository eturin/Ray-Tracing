use rand::Rng;
use utils::camera::Camera;
//use utils::cone::Cone;
use utils::Hitable;
use utils::dielectric::Dielectric;
use utils::lambertian::Lambertian;
use utils::metal::Metal;
use {
    std::{
        fs::File,
        io::{BufWriter, Write},
    },
    utils::figs::Figs,
    utils::ray::Ray,
    utils::sphere::Sphere,
    utils::vec3::Vec3,
};

fn color(r: &Ray, world: &Figs, depth: u8) -> Vec3 {
    // Базовый случай - превышена максимальная глубина рекурсии
    if depth >= 50 {
        return Vec3::default(); // черный
    }

    // Проверка пересечения с объектами сцены
    match world.hit(r, 0.001, f64::MAX) {
        Some(rec) => {
            // Обработка материала
            rec.material
                .and_then(|mat| mat.scatter(r, &rec))
                .map(|(attenuation, scattered)| attenuation * color(&scattered, world, depth + 1))
                .unwrap_or_default()
        }
        // Фон (градиент неба)
        None => {
            let unit_direction = r.direction().make_unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.);
            (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
        }
    }
}

fn random_sceme() -> Figs {
    let n = 500;
    let mut hitable: Vec<Box<dyn Hitable>> = Vec::with_capacity(n + 1);
    hitable.push(Box::new(Sphere::new(
        Vec3::new(0., -1000., -1.),
        1000.,
        Some(Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))),
    )));

    let mut rng = rand::rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random_range(0.0..1.0);
            let center = Vec3::new(
                a as f64 + 0.9 * rng.random_range(0.0..1.0),
                0.2,
                b as f64 + 0.9 * rng.random_range(0.0..1.0),
            );
            if (center - Vec3::new(4., 0.2, 0.)).len() > 0.9 {
                if choose_mat < 0.8 {
                    hitable.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Some(Box::new(Lambertian::new(Vec3::new(
                            rng.random_range(0.0..1.0) * rng.random_range(0.0..1.0),
                            rng.random_range(0.0..1.0) * rng.random_range(0.0..1.0),
                            rng.random_range(0.0..1.0) * rng.random_range(0.0..1.0),
                        )))),
                    )));
                } else if choose_mat < 0.95 {
                    hitable.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Some(Box::new(Metal::new(
                            Vec3::new(
                                0.5 * (1. + rng.random_range(0.0..1.0)),
                                0.5 * (1. + rng.random_range(0.0..1.0)),
                                0.5 * rng.random_range(0.0..1.0),
                            ),
                            0.3,
                        ))),
                    )));
                } else {
                    hitable.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Some(Box::new(Dielectric::new(1.5))),
                    )));
                }
            }
        }
    }

    hitable.push(Box::new(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.,
        Some(Box::new(Dielectric::new(1.5))),
    )));
    hitable.push(Box::new(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.,
        Some(Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)))),
    )));
    hitable.push(Box::new(Sphere::new(
        Vec3::new(4., 1., 0.),
        1.,
        Some(Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))),
    )));

    Figs { v: hitable }
}
fn render(buf_writer: &mut BufWriter<File>) -> Result<(), String> {
    const IMAGE_WIDTH: usize = 1200;
    const IMAGE_HEIGHT: usize = 800;
    const NS: usize = 100;

    let world = random_sceme();

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., -0.);
    let cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0., 1., 0.),
        20.,
        IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64,
        0.1,
        10.,
    );
    if let Err(e) = write!(buf_writer, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n") {
        return Err(format!("Ошибка записи в файл: {e:?}"));
    }
    let mut rng = rand::rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut col = Vec3::default();
            for _s in 0..NS {
                let (random_a, random_b): (f64, f64) =
                    (rng.random_range(0.0..1.0), rng.random_range(0.0..1.0));
                let u = (i as f64 + random_a) / IMAGE_WIDTH as f64;
                let v = (j as f64 + random_b) / IMAGE_HEIGHT as f64;
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            col /= NS as f64;
            col.sqrt();
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
        .open("chap-12.ppm")
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
