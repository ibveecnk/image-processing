/// An rgba pixel
#[derive(Debug, Clone, Copy)]
pub struct Rgba {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
    /// Alpha
    pub a: u8,
}

impl From<&[u8]> for Rgba {
    fn from(val: &[u8]) -> Self {
        Self {
            r: val[0],
            g: val[1],
            b: val[2],
            a: val[3],
        }
    }
}

/// Convert a slice of rgba pixels to a u8 slice
pub fn vec_to_u8_slice(rgba: &[Rgba]) -> Vec<u8> {
    rgba.iter()
        .flat_map(|pixel| vec![pixel.r, pixel.g, pixel.b, pixel.a])
        .collect()
}
