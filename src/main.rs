use image::{GenericImageView, Luma, Pixel};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::result::Result;
use structopt::StructOpt;

mod svg;

use crate::svg::{circle, diamond, g, rect, svg};

#[derive(StructOpt)]
/// Create SVG halftone patterns from raster images
pub struct Options {
    /// Input raster image (png, jpg, gif)
    pub file: String,

    #[structopt(long, short, default_value = "out.svg")]
    /// Output path
    pub output: String,

    #[structopt(long, default_value = "300")]
    /// Output width in mm
    pub output_width: f64,

    #[structopt(long, short, default_value = "5")]
    /// Horizontal spacing between samples in mm
    pub spacing: f64,

    #[structopt(long, default_value = "circle")]
    /// Shape used for samples. "circle" or "diamond"
    pub shape: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let options: Options = Options::from_args();

    let img = image::open(options.file)?;

    let image_width = img.width() as f64;
    let image_height = img.height() as f64;

    let image_ratio = image_width / image_height;

    let spacing = options.spacing;
    let output_width = options.output_width;
    let output_height = output_width * image_ratio;

    let spacing_ratio = 1.0;
    let spacing_x = spacing;
    let spacing_y = spacing_x * spacing_ratio;

    let samples_width = (output_width / spacing) as u32;
    let samples_height = (samples_width as f64 * image_ratio * spacing_ratio) as u32;

    let samples_width_f = samples_width as f64;
    let samples_height_f = samples_height as f64;

    let mut samples = Vec::new();

    for x in 1..samples_width {
        for y in 1..samples_height {
            let pixel_x = (x as f64 / samples_width_f) * image_width;
            let pixel_y = (y as f64 / samples_height_f) * image_height;
            let pixel: Luma<u8> = img.get_pixel(pixel_x as u32, pixel_y as u32).to_luma();
            let radius = (pixel.data[0] as f64 / 255.0) * spacing_x * 0.45;

            if radius < 0.08 {
                continue;
            }

            let sample_x = x as f64 * spacing_x;
            let sample_y = y as f64 * spacing_y;

            let sample = match &*options.shape {
                "diamond" => diamond(sample_x.into(), sample_y.into(), radius),
                "circle" | _ => circle(sample_x.into(), sample_y.into(), radius),
            };
            samples.push(sample);
        }
    }

    let data = svg(
        vec![
            ("width", format!("{}mm", output_width)),
            ("height", format!("{}mm", output_height)),
            (
                "viewBox",
                format!("0 0 {} {}", output_width, output_height),
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
        let file = File::create(&options.output)?;
        let mut f = BufWriter::new(file);
        writeln!(
            f,
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#
        )?;
        write!(f, "{}", data)?;
    }
    println!("Output written to {}", options.output);

    Ok(())
}
