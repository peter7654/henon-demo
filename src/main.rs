use env_logger;

mod cli;
mod imagegen;

use cli::parse_args;
use imagegen::{Henon, Bounds, henon_to_png_multisample_fixed_minmax, generate_image_series_for_a};

fn main() -> std::io::Result<()> {
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Info)
        .init();
    let (a, b, x0, y0, n, width, height, factor, no_image, a_range, a_steps) = parse_args();

    let bounds = Bounds {
        min_x: -1.2846632612345517,
        max_x: 1.2729723934315698,
        min_y: -0.38539897837036546,
        max_y: 0.3818917180294709,
    };

    if let (Some(a_range), Some(a_steps)) = (a_range, a_steps) {
        generate_image_series_for_a(a, a_range, a_steps, b, x0, y0, n, width, height, factor, no_image, &bounds);
    } else {
        let henon = Henon { a, b };
        let filename = format!("henon_{a}_{b}_{x0}_{y0}_{n}_{width}x{height}_f{factor}.png");
        if !no_image {
            let start_img = std::time::Instant::now();
            henon_to_png_multisample_fixed_minmax(
                &henon, x0, y0, n, width, height, factor, &filename,
                &bounds
            );
            let img_duration = start_img.elapsed();
            log::info!("Image saved as: {} (Creation took: {})", filename, humantime::format_duration(img_duration));
        }
    }
    Ok(())
}
