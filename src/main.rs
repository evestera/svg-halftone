use std::fmt;
use std::fs::File;
use std::io::Write;
use image::{
    GenericImageView,
    Pixel,
    Luma,
};

struct Element {
    name: &'static str,
    attributes: Vec<(&'static str, String)>,
    children: Vec<Element>,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}", self.name)?;
        for (name, val) in self.attributes.iter() {
            write!(f, " {}=\"{}\"", name, val)?;
        }
        if self.children.is_empty() {
            write!(f, "/>")?;
        } else {
            write!(f, ">")?;
            for element in self.children.iter() {
                write!(f, "{}\n", element)?;
            }
            write!(f, "</{}>", self.name)?;
        }
        Ok(())
    }
}

fn circle(cx: f64, cy: f64, r: f64) -> Element {
    Element {
        name: "circle",
        attributes: vec![
            ("cx", cx.to_string()),
            ("cy", cy.to_string()),
            ("r", r.to_string()),
        ],
        children: vec![],
    }
}

fn g(attributes: Vec<(&'static str, String)>, children: Vec<Element>) -> Element {
    Element {
        name: "g",
        attributes,
        children,
    }
}

fn svg(attributes: Vec<(&'static str, String)>, children: Vec<Element>) -> Element {
    Element {
        name: "svg",
        attributes,
        children,
    }
}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let samples_width = 100;
    let samples_height = 100;
    let samples_width_f = samples_width as f64;
    let samples_height_f = samples_height as f64;

    let img = image::open("avatar.png")?;
    println!("Image loaded");

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
    println!("Samples created");

    let data =
        svg(vec![
            ("width", "300mm".into()),
            ("height", "300mm".into()),
            ("viewBox", format!("0 0 {} {}", samples_width, samples_height)),
            ("xmlns", "http://www.w3.org/2000/svg".into()),
        ], vec![
            Element {
                name: "rect",
                attributes: vec![
                    ("width", "100%".into()),
                    ("height", "100%".into()),
                    ("fill", "black".into()),
                ],
                children: vec![]
            },
            g(vec![
                ("fill", "white".into()),
            ], samples),
        ]);
    println!("SVG AST created");

    let mut f = File::create("out.svg")?;
    write!(f, r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>{}"#, data)?;
    println!("SVG written");

    Ok(())
}
