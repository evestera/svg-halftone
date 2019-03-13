use image::{GenericImageView, Luma, Pixel};

mod grid;
mod options;
mod poisson;
mod svg;

pub use options::{Grid, Options, Shape};
pub use svg::Element;

pub fn create_halftone_svg(options: Options) -> Element {
    let mut img = options.image;

    if options.invert {
        img.invert();
    }
    if let Some(contrast) = options.contrast {
        img = img.adjust_contrast(contrast);
    }

    let image_width = img.width() as f64;
    let image_height = img.height() as f64;

    let image_ratio = image_width / image_height;

    let shape = options.shape;

    let spacing = options.spacing;
    let output_width = options.output_width;
    let output_height = output_width / image_ratio;

    let resolution_ratio = output_width / image_width;

    let mut samples = Vec::new();

    let coords = match options.grid {
        Grid::Rect => grid::rect(output_width, output_height, spacing),
        Grid::Hex => grid::hex(output_width, output_height, spacing),
        Grid::Diamond => grid::diamond(output_width, output_height, spacing),
        Grid::Poisson => poisson::poisson(output_width, output_height, spacing),
    };

    for (x, y) in coords {
        let pixel_x = (x / resolution_ratio) as u32;
        let pixel_y = (y / resolution_ratio) as u32;
        let pixel: Luma<u8> = img.get_pixel(pixel_x, pixel_y).to_luma();
        let radius = (pixel.data[0] as f64 / 255.0) * spacing * 0.45;

        if radius < 0.08 {
            continue;
        }

        let sample = match shape {
            Shape::Diamond => svg::diamond(x, y, radius),
            Shape::Hex => svg::hex(x, y, radius),
            Shape::Circle => svg::circle(x, y, radius),
        };
        samples.push(sample);
    }

    svg::svg(
        vec![
            ("width", format!("{}mm", output_width)),
            ("height", format!("{}mm", output_height)),
            ("viewBox", format!("0 0 {} {}", output_width, output_height)),
            ("xmlns", "http://www.w3.org/2000/svg".into()),
        ],
        if options.cut_paths {
            svg::cut_paths(samples)
        } else if options.invert {
            svg::black_on_white(samples)
        } else {
            svg::white_on_black(samples)
        },
    )
}
