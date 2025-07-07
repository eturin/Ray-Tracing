use rand::Rng;
use utils::camera::Camera;
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

fn color(r: &Ray, world: &Figs) -> Vec3 {
    if let Some(rec) = world.hit(r, 0., f64::MAX) {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        return 0.5 * color(&Ray::new(rec.p, target - rec.p), world);
    }

    let unit_direction = r.direction().make_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
}
fn render(buf_writer: &mut BufWriter<File>) -> Result<(), String> {
    const IMAGE_WIDTH: usize = 200;
    const IMAGE_HEIGHT: usize = 100;
    const NS: usize = 100;

    let mut world = Figs { v: Vec::new() };
    world
        .v
        .push(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world
        .v
        .push(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));
    let cam = Camera::default();
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
                col += color(&r, &world);
            }
            col /= NS as f64;
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
        .open("chap-7-1.ppm")
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
