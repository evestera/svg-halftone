use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::result::Result;
use std::str::FromStr;
use structopt::StructOpt;
use svg_halftone_lib::{create_halftone_svg, Grid, Options, Shape, image};

#[derive(StructOpt)]
#[structopt(rename_all = "kebab_case")]
/// Create SVG halftone patterns from raster images
pub struct CliOptions {
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

    #[structopt(long, parse(try_from_str = "Shape::from_str"))]
    /// Shape used for samples. "circle", "hex" or "diamond". Defaults to the shape
    /// best suited to the chosen grid.
    pub shape: Option<Shape>,

    #[structopt(long, parse(try_from_str = "Grid::from_str"))]
    /// Grid to lay samples out on. "rect", "hex", "diamond" or "poisson". Defaults to the grid
    /// best suited to the chosen shape.
    pub grid: Option<Grid>,

    #[structopt(long)]
    /// Make shapes black on white. I.e. holes show a darker background.
    pub invert: bool,

    #[structopt(long)]
    /// Draw cut paths only (no fill and background). I.e. make a file ready for cutting.
    pub cut_paths: bool,

    #[structopt(long, raw(allow_hyphen_values = "true"))]
    /// Adjust contrast of input image before processing.
    /// Positive numbers increase contrast, negative numbers decrease it.
    pub contrast: Option<f32>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli_options: CliOptions = CliOptions::from_args();

    let img = image::open(cli_options.file)?;

    let (shape, grid) = match (cli_options.shape, cli_options.grid) {
        (Some(shape), Some(grid)) => (shape, grid),
        (Some(shape), None) => (shape, Grid::from(shape)),
        (None, Some(grid)) => (Shape::from(grid), grid),
        (None, None) => (Shape::Circle, Grid::Rect),
    };

    let data = create_halftone_svg(Options {
        image: img,
        output_width: cli_options.output_width,
        spacing: cli_options.spacing,
        shape,
        grid,
        invert: cli_options.invert,
        cut_paths: cli_options.cut_paths,
        contrast: cli_options.contrast,
    });

    {
        let file = File::create(&cli_options.output)?;
        let mut f = BufWriter::new(file);
        writeln!(
            f,
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#
        )?;
        write!(f, "{}", data)?;
    }
    println!("Output written to {}", cli_options.output);

    Ok(())
}
