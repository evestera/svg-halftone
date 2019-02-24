use image::{GenericImageView, Luma, Pixel};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::result::Result;
use structopt::StructOpt;

mod grid;
mod poisson;
mod svg;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab_case")]
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
    /// Shape used for samples. "circle", "hex" or "diamond"
    pub shape: String,

    #[structopt(long, default_value = "rect")]
    /// Grid to lay samples out on. "rect", "hex", "diamond" or "poisson"
    pub grid: String,
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

    let resolution_ratio = output_width / image_width;

    let mut samples = Vec::new();

    let coords = match &*options.grid {
        "rect" => grid::rect(output_width, output_height, spacing),
        "hex" => grid::hex(output_width, output_height, spacing),
        "diamond" => grid::diamond(output_width, output_height, spacing),
        "poisson" | _ => poisson::poisson(output_width, output_height, spacing),
    };

    for (x, y) in coords {
        let pixel_x = x / resolution_ratio;
        let pixel_y = y / resolution_ratio;
        let pixel: Luma<u8> = img.get_pixel(pixel_x as u32, pixel_y as u32).to_luma();
        let radius = (pixel.data[0] as f64 / 255.0) * spacing * 0.45;

        if radius < 0.08 {
            continue;
        }

        let sample = match &*options.shape {
            "diamond" => svg::diamond(x, y, radius),
            "hex" => svg::hex(x, y, radius),
            "circle" | _ => svg::circle(x, y, radius),
        };
        samples.push(sample);
    }

    let data = svg::svg(
        vec![
            ("width", format!("{}mm", output_width)),
            ("height", format!("{}mm", output_height)),
            ("viewBox", format!("0 0 {} {}", output_width, output_height)),
            ("xmlns", "http://www.w3.org/2000/svg".into()),
        ],
        vec![
            svg::rect(
                vec![
                    ("width", "100%".into()),
                    ("height", "100%".into()),
                    ("fill", "black".into()),
                ],
                vec![],
            ),
            svg::g(vec![("fill", "white".into())], samples),
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
