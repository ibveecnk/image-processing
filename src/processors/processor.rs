use photon_rs::PhotonImage;

/// Image processing trait
pub trait Processor {
    /// Process the image
    fn process(&self, img: &PhotonImage) -> PhotonImage;
}
