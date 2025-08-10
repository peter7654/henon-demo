use clap::{Arg, Command};
use log::info;

pub struct Defaults {
    pub a: &'static str,
    pub b: &'static str,
    pub x0: &'static str,
    pub y0: &'static str,
    pub n: &'static str,
    pub width: &'static str,
    pub height: &'static str,
    pub factor: &'static str,
    pub no_image: bool,
    pub a_range: &'static str,
    pub a_steps: &'static str,
}

pub const DEFAULTS: Defaults = Defaults {
    a: "1.4",
    b: "0.3",
    x0: "0.0",
    y0: "0.0",
    n: "10000000",
    width: "1920",
    height: "1080",
    factor: "4",
    no_image: false,
    a_range: "",
    a_steps: "",
};

pub fn parse_args() -> (f64, f64, f64, f64, u32, u32, u32, u32, bool, Option<f64>, Option<u32>) {
    let matches = Command::new("henon_single")
        .about("This program computes and visualizes the trajectory of the Hénon map, a well-known discrete-time dynamical system exhibiting chaotic behavior. It supports generating high-resolution images of the attractor, multisampling for quality, and batch creation of image series for parameter studies. Output is in PNG format, and all parameters are configurable via command line.")
        .arg(Arg::new("a").long("a").value_parser(clap::value_parser!(f64)).default_value(DEFAULTS.a).help("Parameter a"))
        .arg(Arg::new("b").long("b").value_parser(clap::value_parser!(f64)).default_value(DEFAULTS.b).help("Parameter b"))
        .arg(Arg::new("x0").long("x0").value_parser(clap::value_parser!(f64)).default_value(DEFAULTS.x0).help("Initial value x0"))
        .arg(Arg::new("y0").long("y0").value_parser(clap::value_parser!(f64)).default_value(DEFAULTS.y0).help("Initial value y0"))
        .arg(Arg::new("n").long("n").value_parser(clap::value_parser!(u32)).default_value(DEFAULTS.n).help("Number of iterations"))
        .arg(Arg::new("width").long("width").value_parser(clap::value_parser!(u32)).default_value(DEFAULTS.width).help("Image width in pixels"))
        .arg(Arg::new("height").long("height").value_parser(clap::value_parser!(u32)).default_value(DEFAULTS.height).help("Image height in pixels"))
        .arg(Arg::new("factor").long("factor").value_parser(clap::value_parser!(u32)).default_value(DEFAULTS.factor).help("Multisampling factor"))
        .arg(Arg::new("no-image").long("no-image").action(clap::ArgAction::SetTrue).default_value(if DEFAULTS.no_image {"true"} else {"false"}).help("Disable image output"))
        .arg(Arg::new("a-range").long("a-range").value_parser(clap::value_parser!(f64)).help("Value for a variation (range from a to a+a-range)"))
        .arg(Arg::new("a-steps").long("a-steps").value_parser(clap::value_parser!(u32)).help("Number of steps for a (for image series)"))
        .get_matches();
    let a = *matches.get_one::<f64>("a").unwrap();
    let b = *matches.get_one::<f64>("b").unwrap();
    let x0 = matches.get_one::<f64>("x0").copied().unwrap_or_else(|| {
        log::error!("Error parsing x0");
        std::process::exit(1);
    });
    let y0 = matches.get_one::<f64>("y0").copied().unwrap_or_else(|| {
        log::error!("Error parsing y0");
        std::process::exit(1);
    });
    let n = matches.get_one::<u32>("n").copied().unwrap_or_else(|| {
        log::error!("Error parsing n");
        std::process::exit(1);
    });
    let width = matches.get_one::<u32>("width").copied().unwrap_or_else(|| {
        log::error!("Error parsing width");
        std::process::exit(1);
    });
    let height = matches.get_one::<u32>("height").copied().unwrap_or_else(|| {
        log::error!("Error parsing height");
        std::process::exit(1);
    });
    let factor = matches.get_one::<u32>("factor").copied().unwrap_or_else(|| {
        log::error!("Error parsing factor");
        std::process::exit(1);
    });
    let no_image = matches.get_flag("no-image");
    let a_range = matches.get_one::<f64>("a-range").copied();
    let a_steps = matches.get_one::<u32>("a-steps").copied();
    // Logging if value differs from default
    if a != DEFAULTS.a.parse::<f64>().unwrap_or(f64::NAN) { info!("Parameter a overwritten: {}", a); }
    if b != DEFAULTS.b.parse::<f64>().unwrap_or(f64::NAN) { info!("Parameter b overwritten: {}", b); }
    if x0 != DEFAULTS.x0.parse::<f64>().unwrap_or(f64::NAN) { info!("Parameter x0 overwritten: {}", x0); }
    if y0 != DEFAULTS.y0.parse::<f64>().unwrap_or(f64::NAN) { info!("Parameter y0 overwritten: {}", y0); }
    if n != DEFAULTS.n.parse::<u32>().unwrap_or(u32::MAX) { info!("Parameter n overwritten: {}", n); }
    if width != DEFAULTS.width.parse::<u32>().unwrap_or(u32::MAX) { info!("Parameter width overwritten: {}", width); }
    if height != DEFAULTS.height.parse::<u32>().unwrap_or(u32::MAX) { info!("Parameter height overwritten: {}", height); }
    if factor != DEFAULTS.factor.parse::<u32>().unwrap_or(u32::MAX) { info!("Parameter factor overwritten: {}", factor); }
    if no_image != DEFAULTS.no_image { info!("Parameter no-image overwritten: {}", no_image); }
    if let Some(val) = a_range {
        if !DEFAULTS.a_range.is_empty() {
            let def_val = DEFAULTS.a_range.parse::<f64>().unwrap_or(f64::NAN);
            if val != def_val { info!("Parameter a-range overwritten: {}", val); }
        } else {
            info!("Parameter a-range set: {}", val);
        }
    }
    if let Some(val) = a_steps {
        if !DEFAULTS.a_steps.is_empty() {
            let def_val = DEFAULTS.a_steps.parse::<u32>().unwrap_or(u32::MAX);
            if val != def_val { info!("Parameter a-steps overwritten: {}", val); }
        } else {
            info!("Parameter a-steps set: {}", val);
        }
    }
    (
        a, b, x0, y0, n, width, height, factor, no_image, a_range, a_steps
    )
}
