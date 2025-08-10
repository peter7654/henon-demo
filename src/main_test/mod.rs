#[cfg(test)]
mod tests {
    use crate::imagegen::{Henon, henon_to_png_multisample_fixed_minmax};
    use std::fs;
    use std::path::Path;

    #[test]
    fn henon_step_basic() {
        let henon = Henon { a: 1.4, b: 0.3 };
        let (x1, y1) = henon.step(0.0, 0.0);
        assert!((x1 - 1.0).abs() < 1e-10);
        assert!((y1 - 0.0).abs() < 1e-10);
        let (x2, y2) = henon.step(x1, y1);
        assert!((x2 - (1.0 - 1.4 * x1 * x1)).abs() < 1e-10);
        assert!((y2 - 0.3 * x1).abs() < 1e-10);
    }

    #[test]
    fn henon_image_generation_creates_file() {
        let henon = Henon { a: 1.4, b: 0.3 };
        let filename = "test_henon.png";
        henon_to_png_multisample_fixed_minmax(
            &henon, 0.0, 0.0, 1000, 100, 100, 1, filename,
            -1.5, 1.5, -0.5, 0.5
        );
        assert!(Path::new(filename).exists());
        let _ = fs::remove_file(filename);
    }
}

