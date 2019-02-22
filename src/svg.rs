use std::fmt;

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
