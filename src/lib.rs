use std::path::Path;

pub enum ResizeMode {
    Fit,
    Fill,
    Stretch,
}

/// Represents a hash of an image.
pub struct ImageHash {
    pub matrix: Vec<Vec<bool>>,
}

impl ImageHash {
    // flattens the bit matrix to a raw vector of bits
    pub fn flatten(&self) -> Vec<bool> {
        self.matrix.iter().flatten().copied().collect()
    }

    // produces a hexadecimal string representation of the hash
    pub fn to_string(&self) -> String {
        "hello".to_string()
    }

    // converts a given string to a hash with the specificed shape
    pub fn from_string(s: &str, width: usize, height: usize) -> ImageHash {
        println!("String: {} - Width: {} - Height: {}", s, width, height);

        ImageHash {
            matrix: vec![vec![false; 8]; 8],
        }
    }
}

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
mod resize;
