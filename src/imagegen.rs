use std::time::Instant;
use log::info;
use humantime::format_duration;
use image::{ExtendedColorType, ImageEncoder};
use rayon::prelude::*;
use fast_image_resize as fir;
use fir::{PixelType};
use fir::images::Image as FirImage;

pub struct Henon {
    pub a: f64,
    pub b: f64,
}

impl Henon {
    #[inline]
    pub fn step(&self, x: f64, y: f64) -> (f64, f64) {
        let x_next = 1.0 - self.a * x * x + y;
        let y_next = self.b * x;
        (x_next, y_next)
    }
}

pub struct Bounds {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

pub fn resize_image<'a>(src_image: &'a fir::images::Image<'a>, width: u32, height: u32) -> fir::images::Image<'a> {
    use fast_image_resize as fir;
    use fir::{ResizeAlg, FilterType};
    let mut dst_image = fir::images::Image::new(width, height, fir::PixelType::U8);
    let mut resizer = fir::Resizer::new();
    #[cfg(target_arch = "x86_64")]
    unsafe {
        resizer.set_cpu_extensions(fir::CpuExtensions::Sse4_1);
    }
    resizer.resize(src_image, &mut dst_image, &fir::ResizeOptions::new().resize_alg(ResizeAlg::Convolution(FilterType::Lanczos3))).unwrap();
    dst_image
}

pub fn write_png(filename: &str, buffer: &[u8], width: u32, height: u32) -> std::io::Result<()> {
    use image::codecs::png::PngEncoder;
    use std::fs::File;
    use std::io::{BufWriter, Write};
    let mut result_buffer = BufWriter::new(Vec::new());
    PngEncoder::new(&mut result_buffer)
        .write_image(buffer, width, height, ExtendedColorType::L8)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("PNG encoding error: {e}")))?;
    let mut file = File::create(filename)?;
    result_buffer.flush()?;
    file.write_all(result_buffer.get_ref())?;
    Ok(())
}

pub fn henon_to_png_multisample_fixed_minmax(
    henon: &Henon,
    x0: f64,
    y0: f64,
    n: u32,
    width: u32,
    height: u32,
    factor: u32,
    filename: &str,
    bounds: &Bounds
) {
    let super_width = width * factor;
    let super_height = height * factor;
    let range_x = bounds.max_x - bounds.min_x;
    let range_y = bounds.max_y - bounds.min_y;
    let scale_x = 0.9 * super_width as f64 / range_x;
    let scale_y = 0.9 * super_height as f64 / range_y;
    let scale = scale_x.min(scale_y);
    let used_width = scale * range_x;
    let used_height = scale * range_y;
    let offset_x = ((super_width as f64 - used_width) / 2.0).max(0.0);
    let offset_y = ((super_height as f64 - used_height) / 2.0).max(0.0);
    let mut src_image = FirImage::new(super_width, super_height, PixelType::U8);
    src_image.buffer_mut().fill(0);
    info!("Super-sampling: {}x{} pixels, buffer size: {} bytes (grayscale)", super_width, super_height, src_image.buffer().len());
    let start_calc = Instant::now();
    let (mut x, mut y) = (x0, y0);
    for _ in 0..=n {
        let px = ((x - bounds.min_x) * scale + offset_x).round() as u32;
        let py = ((y - bounds.min_y) * scale + offset_y).round() as u32;
        if px < super_width && py < super_height {
            src_image.buffer_mut()[(py * super_width + px) as usize] = 255;
        }
        let (next_x, next_y) = henon.step(x, y);
        x = next_x;
        y = next_y;
    }
    let calc_duration = start_calc.elapsed();
    let avg_duration = calc_duration / n;
    info!("Trajectory calculation completed: {} points, average duration per point: {} ", n + 1, format_duration(avg_duration));
    let dst_image = resize_image(&src_image, width, height);
    if let Err(e) = write_png(filename, dst_image.buffer(), width, height) {
        log::error!("Error writing PNG file {}: {}", filename, e);
    }
}

pub fn generate_image_series_for_a(
    a_center: f64,
    a_range: f64,
    a_steps: u32,
    b: f64,
    x0: f64,
    y0: f64,
    n: u32,
    width: u32,
    height: u32,
    factor: u32,
    no_image: bool,
    bounds: &Bounds
) {
    use std::fs;
    use std::path::Path;
    let a_start = a_center - a_range / 2.0;
    let a_step = a_range / (a_steps - 1) as f64;
    let steps_dir = "steps";
    if !Path::new(steps_dir).exists() {
        if let Err(e) = fs::create_dir(steps_dir) {
            log::error!("Could not create steps directory: {}", e);
            return;
        }
    }
    (0..a_steps).into_par_iter().for_each(|i| {
        let a_val = a_start + a_step * i as f64;
        let henon = Henon { a: a_val, b };
        let filename = format!("{}/henon_a{:.6}_b{:.6}_x{}_y{}_n{}_{}x{}_f{}_step{}.png", steps_dir, a_val, b, x0, y0, n, width, height, factor, i);
        if !no_image {
            let start_img = Instant::now();
            match std::panic::catch_unwind(|| {
                henon_to_png_multisample_fixed_minmax(
                    &henon, x0, y0, n, width, height, factor, &filename,
                    bounds
                );
            }) {
                Err(e) => {
                    log::error!("Error creating image: {} | Parameter a: {:.6}, b: {:.6}, step: {} | Error: {:?}", filename, a_val, b, i, e);
                },
                Ok(_) => {
                    let img_duration = start_img.elapsed();
                    info!("Image saved as: {} | Parameter a: {:.6}, b: {:.6}, step: {} | Duration: {}", filename, a_val, b, i, format_duration(img_duration));
                }
            }
        }
    });
}
