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
    fn hash_from_path(&self, path: &Path) -> ImageHash;

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

pub mod average;
mod convert;
mod imghash;

pub use crate::imghash::ImageHash;
