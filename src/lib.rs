use std::path::Path;

/// Represents a hash of an image.
pub struct Hash {
    pub matrix: Vec<Vec<bool>>,
}

impl Hash {
    pub fn flatten(&self) -> Vec<bool> {
        self.matrix.iter().flatten().copied().collect()
    }
}

/// Trait for generating image hashes.
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
    fn hash_path(&self, path: Path) -> Hash;

    /// Generates a hash for a given image.
    ///
    /// # Arguments
    ///
    /// * `img` - The image to generate the hash for.
    ///
    /// # Returns
    ///
    /// The generated image hash.
    fn hash_img(&self, img: &image::DynamicImage) -> Hash;
}
