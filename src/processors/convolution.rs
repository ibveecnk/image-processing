#![allow(dead_code)]
use super::processor::Processor;

/// Edge detection
const EDGE_DETECTION: [[i32; 3]; 3] = [[-1, -1, -1], [-1, 8, -1], [-1, -1, -1]];

/// Convolution kernel
const KERNEL: [[i32; 3]; 3] = EDGE_DETECTION;

/// Convolution processor
pub struct Convolution;

impl Processor for Convolution {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_lossless,
        clippy::cast_possible_wrap
    )]
    fn process(&self, img: &photon_rs::PhotonImage) -> photon_rs::PhotonImage {
        let pixels = img
            .get_raw_pixels()
            .chunks_exact(4)
            .map(std::convert::Into::into)
            .collect::<Vec<crate::rgba::Rgba>>();

        let width = img.get_width() as usize;
        let height = img.get_height() as usize;

        let mut new_pixels = Vec::with_capacity(pixels.len());

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let pixel = &pixels[idx];

                let mut red = 0_i32;
                let mut green = 0_i32;
                let mut blue = 0_i32;

                for (ky, kern) in KERNEL.iter().enumerate() {
                    for (kx, inner_kern) in kern.iter().enumerate() {
                        let x: i32 = x as i32 + kx as i32 - 1;
                        let y: i32 = y as i32 + ky as i32 - 1;

                        if x < 0
                            || x >= (width as u32)
                                .try_into()
                                .expect("Should be able to convert")
                            || y < 0
                            || y >= (height as u32)
                                .try_into()
                                .expect("Should be able to convert")
                        {
                            continue;
                        }

                        let idx = y as usize * width + x as usize;
                        let pixel = &pixels[idx];

                        red += pixel.r as i32 * (*inner_kern);
                        green += pixel.g as i32 * (*inner_kern);
                        blue += pixel.b as i32 * (*inner_kern);
                    }
                }

                let r = red.clamp(0, 255) as u8;
                let g = green.clamp(0, 255) as u8;
                let b = blue.clamp(0, 255) as u8;

                new_pixels.push(crate::rgba::Rgba {
                    r,
                    g,
                    b,
                    a: pixel.a,
                });
            }
        }

        photon_rs::PhotonImage::new(
            crate::rgba::vec_to_u8_slice(&new_pixels),
            width as u32,
            height as u32,
        )
    }
}
