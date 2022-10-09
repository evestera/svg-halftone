use cfg_if::cfg_if;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use svg_halftone_lib::{create_halftone_svg, image, Grid, Options, Shape};

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn run(
    bytes: &[u8],
    output_width: f64,
    spacing: f64,
    shape: &str,
    grid: &str,
    invert: bool,
    cut_paths: bool,
) -> Option<String> {
    set_panic_hook();

    let img = match image::load_from_memory(bytes) {
        Ok(img) => img,
        Err(_) => return None
    };

    let (shape, grid) = match (Shape::from_str(shape).ok(), Grid::from_str(grid).ok()) {
        (Some(shape), Some(grid)) => (shape, grid),
        (Some(shape), None) => (shape, Grid::from(shape)),
        (None, Some(grid)) => (Shape::from(grid), grid),
        (None, None) => (Shape::Circle, Grid::Rect),
    };

    let data = create_halftone_svg(Options {
        image: img,
        output_width,
        spacing,
        shape,
        grid,
        invert,
        cut_paths,
        contrast: None,
        multi_sample: true,
    });

    Some(format!("{}", data))
}
