// Poisson Disk Sampling
// implementation based on Fast Poisson Disk Sampling in Arbitrary Dimensions
// https://www.cs.ubc.ca/~rbridson/docs/bridson-siggraph07-poissondisk.pdf

use std::f64::consts::PI;
use rand::prelude::*;

const K: u64 = 30;

pub fn poisson(width: f64, height: f64, spacing: f64) -> Box<dyn Iterator<Item = (f64, f64)>> {
    let r = spacing;
    let half_r = r / 2.0;
    let cell_size = r / 2.0_f64.sqrt();

    let cells_w = (width / cell_size).ceil() as usize;
    let cells_h = (height / cell_size).ceil() as usize;

    // using 0 as empty rather than -1
    let mut background_grid: Vec<Vec<usize>> = vec![vec![0; cells_h]; cells_w];

    let mut rng = SmallRng::from_entropy();

    let mut samples: Vec<(f64, f64)> = vec![(-1.0, -1.0)]; // dummy at index 0
    let mut active: Vec<usize> = Vec::new();

    let initial = (rng.gen_range(half_r, width - half_r), rng.gen_range(half_r, height - half_r));

    add_sample(
        &mut samples,
        &mut active,
        &mut background_grid,
        cell_size,
        initial,
    );

    'outer: while !active.is_empty() {
        let curr_i = rng.gen_range(0, active.len());
        let curr = samples[active[curr_i]];
        for _ in 0..K {
            // create new point with r < distance < 2r from curr
            let magnitude = rng.gen_range(r, 2.0 * r);
            let angle = rng.gen_range(-PI, PI);
            let x = curr.0 + (magnitude * angle.cos());
            let y = curr.1 + (magnitude * angle.sin());
            if x < half_r || y < half_r || x > (width - half_r) || y > (height - half_r) {
                continue;
            }
            let new = (x, y);
            if no_samples_within_r(&background_grid, &samples, cell_size, new, r) {
                add_sample(
                    &mut samples,
                    &mut active,
                    &mut background_grid,
                    cell_size,
                    new,
                );
                continue 'outer;
            }
        }
        active.swap_remove(curr_i);
    }

    Box::new(samples.into_iter().skip(1)) // skip dummy at index 0
}

fn add_sample(
    samples: &mut Vec<(f64, f64)>,
    active: &mut Vec<usize>,
    background_grid: &mut [Vec<usize>],
    cell_size: f64,
    sample: (f64, f64),
) {
    let x = sample.0;
    let y = sample.1;

    let index = samples.len();
    samples.push(sample);
    active.push(index);
    background_grid[(x / cell_size) as usize][(y / cell_size) as usize] = index;
}

fn no_samples_within_r(
    background_grid: &[Vec<usize>],
    samples: &[(f64, f64)],
    cell_size: f64,
    sample: (f64, f64),
    r: f64,
) -> bool {
    let x = sample.0;
    let y = sample.1;

    let i_min = float_to_index(x - r, cell_size, background_grid.len());
    let i_max = float_to_index(x + r, cell_size, background_grid.len());
    let j_min = float_to_index(y - r, cell_size, background_grid[0].len());
    let j_max = float_to_index(y + r, cell_size, background_grid[0].len());

    for i in i_min..=i_max {
        for j in j_min..=j_max {
            let index = background_grid[i][j];
            if index == 0 {
                continue;
            }
            let nearby_sample = samples[index];
            if is_closer_than(r, sample, nearby_sample) {
                return false;
            }
        }
    }

    true
}

fn is_closer_than(min_distance: f64, a: (f64, f64), b: (f64, f64)) -> bool {
    min_distance > distance(a, b)
}

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    let x = a.0 - b.0;
    let y = a.1 - b.1;
    ((x * x) + (y * y)).sqrt()
}

fn float_to_index(f: f64, cell_size: f64, len: usize) -> usize {
    if f < 0.0 {
        return 0;
    }
    let i = (f / cell_size) as usize;
    if i >= len {
        len - 1
    } else {
        i
    }
}
