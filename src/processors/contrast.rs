use photon_rs::PhotonImage;

use crate::rgba::{vec_to_u8_slice, Rgba};

use super::processor::Processor;

/// Contrast processor
pub struct Contrast;

impl Processor for Contrast {
    /// Contrast processing
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_lossless
    )]
    fn process(&self, img: &PhotonImage) -> PhotonImage {
        let mut pixels = img
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

                let r = pixel.r as f32 / 255.0;
                let g = pixel.g as f32 / 255.0;
                let b = pixel.b as f32 / 255.0;

                let r = (r - 0.5).mul_add(1.5, 0.5);
                let g = (g - 0.5).mul_add(1.5, 0.5);
                let b = (b - 0.5).mul_add(1.5, 0.5);

                let r = (r * 255.0).round() as u8;
                let g = (g * 255.0).round() as u8;
                let b = (b * 255.0).round() as u8;

                pixels[idx] = Rgba {
                    r,
                    g,
                    b,
                    a: pixel.a,
                };
            }
        }

        PhotonImage::new(vec_to_u8_slice(&pixels), width as u32, height as u32)
    }
}
