pub fn rect(width: f64, height: f64, spacing: f64) -> Box<dyn Iterator<Item = (f64, f64)>> {
    grid(width, height, spacing, 1.0, false)
}

pub fn hex(width: f64, height: f64, spacing: f64) -> Box<dyn Iterator<Item = (f64, f64)>> {
    grid(width, height, spacing, 0.866, true) // 2 / sqrt(3) ~= 0.866
}

pub fn diamond(width: f64, height: f64, spacing: f64) -> Box<dyn Iterator<Item = (f64, f64)>> {
    grid(width, height, spacing, 0.5, true)
}

fn grid(
    width: f64,
    height: f64,
    spacing: f64,
    spacing_ratio: f64,
    offset: bool,
) -> Box<dyn Iterator<Item = (f64, f64)>> {
    let spacing_x = spacing;
    let spacing_y = spacing_x * spacing_ratio;

    let x_count = (width / spacing_x) as u32;
    let x_remainder = width - (x_count as f64) * spacing_x;
    let mut y_count = (height / spacing_y) as u32;
    let mut y_remainder = height - (y_count as f64) * spacing_y;

    if (spacing_y / 2.0) + (y_remainder / 2.0) < (spacing / 2.0) {
        y_count -= 1;
        y_remainder += spacing_y;
    }

    Box::new((1..=x_count).flat_map(move |x| {
        (1..=y_count)
            .map(move |y| (x, y))
            .filter(move |(x, y)| !offset || y % 2 == 0 || x != &x_count)
            .map(move |(x, y)| {
                let sample_x = (x as f64) * spacing_x - (spacing_x / 2.0)
                    + (x_remainder / 2.0)
                    + if offset && y % 2 != 0 {
                        spacing_x / 2.0
                    } else {
                        0.0
                    };
                let sample_y = (y as f64) * spacing_y - (spacing_y / 2.0) + (y_remainder / 2.0);
                (sample_x, sample_y)
            })
    }))
}
