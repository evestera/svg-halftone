use image::DynamicImage;
use std::str::FromStr;
use std::convert::From;

pub struct Options {
    pub image: DynamicImage,
    pub output_width: f64,
    pub spacing: f64,
    pub shape: Shape,
    pub grid: Grid,
    pub invert: bool,
    pub cut_paths: bool,
    pub contrast: Option<f32>,
    pub multi_sample: bool,
}

#[derive(Copy, Clone)]
pub enum Shape {
    Circle,
    Hex,
    Diamond,
}

impl From<Grid> for Shape {
    fn from(grid: Grid) -> Self {
        match grid {
            Grid::Rect => Shape::Circle,
            Grid::Hex => Shape::Hex,
            Grid::Diamond => Shape::Diamond,
            Grid::Poisson => Shape::Circle,
        }
    }
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "diamond" => Ok(Shape::Diamond),
            "hex" => Ok(Shape::Hex),
            "circle" => Ok(Shape::Circle),
            _ => Err(format!("no shape type named '{}'", s)),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Grid {
    Rect,
    Hex,
    Diamond,
    Poisson,
}

impl From<Shape> for Grid {
    fn from(shape: Shape) -> Self {
        match shape {
            Shape::Circle => Grid::Rect,
            Shape::Hex => Grid::Hex,
            Shape::Diamond => Grid::Diamond,
        }
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rect" => Ok(Grid::Rect),
            "hex" => Ok(Grid::Hex),
            "diamond" => Ok(Grid::Diamond),
            "poisson" => Ok(Grid::Poisson),
            _ => Err(format!("no grid type named '{}'", s)),
        }
    }
}
