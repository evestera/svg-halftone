use std::f64::consts::PI;
use std::fmt;
use std::fmt::Write;

pub struct Element {
    name: &'static str,
    attributes: Vec<(&'static str, String)>,
    children: Vec<Element>,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n<{}", self.name)?;
        for (name, val) in self.attributes.iter() {
            write!(f, " {}=\"{}\"", name, val)?;
        }
        if self.children.is_empty() {
            write!(f, "/>")?;
        } else {
            write!(f, ">")?;
            for element in self.children.iter() {
                write!(f, "{}", element)?;
            }
            write!(f, "</{}>", self.name)?;
        }
        Ok(())
    }
}

pub fn circle(cx: f64, cy: f64, r: f64) -> Element {
    Element {
        name: "circle",
        attributes: vec![
            ("cx", format!("{:.4}", cx)),
            ("cy", format!("{:.4}", cy)),
            ("r", format!("{:.3}", r)),
        ],
        children: vec![],
    }
}

pub fn diamond(cx: f64, cy: f64, r: f64) -> Element {
    Element {
        name: "polygon",
        attributes: vec![(
            "points",
            format!(
                "{:.4},{:.4} {:.4},{:.4} {:.4},{:.4} {:.4},{:.4}",
                cx,
                cy - r,
                cx + r,
                cy,
                cx,
                cy + r,
                cx - r,
                cy
            ),
        )],
        children: vec![],
    }
}

pub fn hex(cx: f64, cy: f64, r: f64) -> Element {
    let mut points = String::new();
    for i in 1..=6 {
        let corner = hex_corner(cx, cy, r, i);
        write!(points, "{:.4},{:.4} ", corner.0, corner.1).unwrap();
    }
    Element {
        name: "polygon",
        attributes: vec![("points", points)],
        children: vec![],
    }
}

fn hex_corner(cx: f64, cy: f64, r: f64, i: u8) -> (f64, f64) {
    let angle: f64 = (i as f64) * PI / 3.0 - PI / 6.0;
    (cx + r * angle.cos(), cy + r * angle.sin())
}

pub fn g(attributes: Vec<(&'static str, String)>, children: Vec<Element>) -> Element {
    Element {
        name: "g",
        attributes,
        children,
    }
}

pub fn svg(attributes: Vec<(&'static str, String)>, children: Vec<Element>) -> Element {
    Element {
        name: "svg",
        attributes,
        children,
    }
}

pub fn rect(attributes: Vec<(&'static str, String)>, children: Vec<Element>) -> Element {
    Element {
        name: "rect",
        attributes,
        children,
    }
}

pub fn cut_paths(samples: Vec<Element>) -> Vec<Element> {
    vec![g(
        vec![
            ("stroke-width", "0.002mm".into()),
            ("stroke", "black".into()),
            ("fill", "none".into()),
        ],
        samples,
    )]
}

pub fn white_on_black(samples: Vec<Element>) -> Vec<Element> {
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
    ]
}

pub fn black_on_white(samples: Vec<Element>) -> Vec<Element> {
    vec![
        rect(
            vec![
                ("width", "100%".into()),
                ("height", "100%".into()),
                ("fill", "white".into()),
            ],
            vec![],
        ),
        g(vec![("fill", "black".into())], samples),
    ]
}
