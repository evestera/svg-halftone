use image::{GenericImageView, Luma, Pixel};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::result::Result;

mod svg;

use crate::svg::{circle, g, rect, svg};

fn main() -> Result<(), Box<dyn Error>> {
    let samples_width = 150;
    let samples_height = 150;
    let samples_width_f = samples_width as f64;
    let samples_height_f = samples_height as f64;

    let img = image::open("avatar.png")?;

    let image_width = img.width() as f64;
    let image_height = img.height() as f64;

    let mut samples = Vec::new();

    for x in 1..samples_width {
        for y in 1..samples_height {
            let pixel_x = (x as f64 / samples_width_f) * image_width;
            let pixel_y = (y as f64 / samples_height_f) * image_height;
            let pixel: Luma<u8> = img.get_pixel(pixel_x as u32, pixel_y as u32).to_luma();
            let radius = (pixel.data[0] as f64 / 255.0) * 0.45;

            if radius < 0.08 {
                continue;
            }

            samples.push(circle(x.into(), y.into(), radius))
        }
    }

    let data = svg(
        vec![
            ("width", "300mm".into()),
            ("height", "300mm".into()),
            (
                "viewBox",
                format!("0 0 {} {}", samples_width, samples_height),
            ),
            ("xmlns", "http://www.w3.org/2000/svg".into()),
        ],
        vec![
            rect(
                vec![
                    ("width", "100%".into()),
                    ("height", "100%".into()),
                    ("fill", "black".into()),
                ],
                vec![],
            ),
            g(vec![("fill", "white".into())], samples),
        ],
    );

    {
        let file = File::create("out.svg")?;
        let mut f = BufWriter::new(file);
        writeln!(
            f,
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#
        )?;
        write!(f, "{}", data)?;
    }

    Ok(())
}
