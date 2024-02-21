use std::fs::File;

use crate::rgba::Rgba;
use std::io::Write;

/// Ascii processor
pub struct Ascii;

impl Ascii {
    /// Print pixel data as ascii to a file
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_lossless
    )]
    pub fn write(img: &photon_rs::PhotonImage) {
        let mut file = File::create("pixel_data.txt").expect("File should open");

        let pixels = img
            .get_raw_pixels()
            .chunks_exact(4)
            .map(std::convert::Into::into)
            .collect::<Vec<Rgba>>();

        let width = img.get_width() as usize;
        let height = img.get_height() as usize;

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let pixel = &pixels[idx];

                let r = pixel.r;
                let g = pixel.g;
                let b = pixel.b;

                let ascii = match (r + g + b) / 3 {
                    0..=42 => " ",
                    43..=85 => ".",
                    86..=128 => "-",
                    129..=171 => "=",
                    172..=214 => "+",
                    215..=255 => "#",
                };

                write!(file, "{ascii}").expect("File should write");
            }
            writeln!(file).expect("File should write");
        }
    }
}
