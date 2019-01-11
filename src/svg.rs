use std::fmt;

pub struct Element {
    pub name: &'static str,
    pub attributes: Vec<(&'static str, String)>,
    pub children: Vec<Element>,
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

pub fn circle(cx: f64, cy: f64, r: f64) -> Element {
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
