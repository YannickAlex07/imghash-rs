use image::ImageError;
use std::path::Path;

/// Trait for generating image hashes
pub trait ImageHasher {
    /// Generates a hash for an image specified by its file path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the image file.
    ///
    /// # Returns
    ///
    /// The generated image hash.
    fn hash_from_path(&self, path: &Path) -> Result<ImageHash, ImageError> {
        match image::io::Reader::open(path)?.decode() {
            Ok(img) => Ok(self.hash_from_img(&img)),
            Err(e) => Err(e),
        }
    }

    /// Generates a hash for a given image.
    ///
    /// # Arguments
    ///
    /// * `img` - The image to generate the hash for.
    ///
    /// # Returns
    ///
    /// The generated image hash.
    fn hash_from_img(&self, img: &image::DynamicImage) -> ImageHash;
}

// public modules
pub mod average;
pub mod difference;
pub mod perceptual;

// private modules
mod convert;
mod imghash;
mod math;

// public exports
pub use crate::imghash::ImageHash;
