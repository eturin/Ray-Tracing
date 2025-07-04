use std::fs::File;
use std::io::{BufWriter, Write};

fn render(buf_writer: &mut BufWriter<File>) -> Result<(), String> {
    const IMAGE_WIDTH: usize = 200;
    const IMAGE_HEIGHT: usize = 100;

    if let Err(e) = write!(buf_writer, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n") {
        return Err(format!("Ошибка записи в файл: {e:?}"));
    }
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / IMAGE_WIDTH as f64;
            let g = j as f64 / IMAGE_HEIGHT as f64;
            let b = 0.2;

            if let Err(e) = writeln!(
                buf_writer,
                "{} {} {}",
                (255.99 * r) as u8,
                (255.99 * g) as u8,
                (255.99 * b) as u8
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
        .open("chap-1.ppm")
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
