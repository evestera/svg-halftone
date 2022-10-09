use image::{DynamicImage, GenericImageView, LumaA, Pixel};

/// Return a number in the range 0.0 - 1.0 representing
/// the "effective luminosity" of the pixel (luma * alpha)
/// at a point mapped to an image pixel
pub fn sample_point(img: &DynamicImage, resolution_ratio: f64, point: (f64, f64)) -> f64 {
    get_pixel_value(&img, clamped_pixel_coords(&img, resolution_ratio, point))
}

/// Return a number in the range 0.0 - 1.0 representing
/// the "effective luminosity" (luma * alpha) of the area
/// around a point (mapped to an image pixel)
pub fn multi_sample_around_point(
    img: &DynamicImage,
    resolution_ratio: f64,
    (x, y): (f64, f64),
    radius: f64,
) -> f64 {
    let sampling_coords = [
        (x, y),
        (x + radius, y),
        (x - radius, y),
        (x, y + radius),
        (x, y - radius),
    ];
    let subsampling_count = sampling_coords.len();
    let sum = sampling_coords
        .iter()
        .map(|xy| sample_point(&img, resolution_ratio, *xy))
        .sum::<f64>();

    sum / subsampling_count as f64
}

/// Return a number in the range 0.0 - 1.0 representing
/// the "effective luminosity" of the pixel (luma * alpha)
fn get_pixel_value(img: &DynamicImage, (pixel_x, pixel_y): (u32, u32)) -> f64 {
    let pixel: LumaA<u8> = img.get_pixel(pixel_x, pixel_y).to_luma_alpha();
    let luma = pixel.data[0] as f64 / 255.0;
    let alpha = pixel.data[1] as f64 / 255.0;
    let sample = luma * alpha;
    sample
}

fn clamped_pixel_coords(
    img: &DynamicImage,
    resolution_ratio: f64,
    (x, y): (f64, f64),
) -> (u32, u32) {
    (
        clamp_as_u32(x / resolution_ratio, img.width() - 1),
        clamp_as_u32(y / resolution_ratio, img.height() - 1),
    )
}

fn clamp_as_u32(n: f64, limit: u32) -> u32 {
    if n < 0.0 {
        0
    } else if n > limit as f64 {
        limit
    } else {
        n as u32
    }
}
