//! # imghash
//!
//! `imghash` provides image hashing algorithms for Rust, compatible with the
//! Python [`imagehash`](https://pypi.org/project/ImageHash/) package.
//!
//! The following hash algorithms are supported:
//!
//! - **Average hash** — compares each pixel to the mean intensity
//! - **Median hash** — compares each pixel to the median intensity
//! - **Difference hash** — compares adjacent pixels in each row
//! - **Perceptual hash** — uses DCT to capture frequency information
//!
//! ## Quick start
//!
//! ```no_run
//! use std::path::Path;
//! use imghash::average_hash;
//!
//! let path = Path::new("path/to/image.png");
//! let hash = average_hash(path).unwrap();
//!
//! // Encode as a hex string
//! let hex = hash.encode().unwrap();
//!
//! // Decode back from hex
//! let decoded = imghash::ImageHash::decode(&hex, 8, 8).unwrap();
//!
//! // Compare two hashes
//! let distance = hash.distance(&decoded).unwrap();
//! assert_eq!(distance, 0);
//! ```
//!
//! ## Custom hashers
//!
//! For more control over hash dimensions and color space, use the hasher structs directly:
//!
//! ```no_run
//! use std::path::Path;
//! use imghash::{average::AverageHasher, ColorSpace, ImageHasher};
//!
//! let hasher = AverageHasher::new(16, 16, ColorSpace::REC601).unwrap();
//! let hash = hasher.hash_from_path(Path::new("path/to/image.png")).unwrap();
//! ```

use average::AverageHasher;
use difference::DifferenceHasher;
use median::MedianHasher;
use perceptual::PerceptualHasher;
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
    fn hash_from_path(&self, path: &Path) -> Result<ImageHash, ImageHashError> {
        let img = image::ImageReader::open(path)
            .map_err(|e| ImageHashError::IoError {
                source: e,
                path: path.to_path_buf(),
            })?
            .decode()?;
        self.hash_from_img(&img)
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
    fn hash_from_img(&self, img: &image::DynamicImage) -> Result<ImageHash, ImageHashError>;
}

/// Calculate the average hash for an image at the specified path. Uses the default
/// width and height of 8 x 8 pixels. If you want to use something else please directly use
/// the [`AverageHasher`] struct.
///
/// # Arguments
/// * `path`: A reference to the path of the image
///
/// # Returns
/// * An [`ImageHash`]-struct that can be encoded into a string representation
/// * An [`ImageHashError`] if something went wrong while loading the image
pub fn average_hash(path: &Path) -> Result<ImageHash, ImageHashError> {
    // create the hasher
    let hasher = AverageHasher::default();

    hasher.hash_from_path(path)
}

/// Calculate the median hash for an image at the specified path. Uses the default
/// width and height of 8 x 8 pixels. If you want to use something else please directly use
/// the [`MedianHasher`] struct.
///
/// # Arguments
/// * `path`: A reference to the path of the image
///
/// # Returns
/// * An [`ImageHash`]-struct that can be encoded into a string representation
/// * An [`ImageHashError`] if something went wrong while loading the image
pub fn median_hash(path: &Path) -> Result<ImageHash, ImageHashError> {
    // create the hasher
    let hasher = MedianHasher::default();

    hasher.hash_from_path(path)
}

/// Calculate the difference hash for an image at the specified path. Uses the default
/// width and height of 8 x 8 pixels. If you want to use something else please directly use
/// the [`DifferenceHasher`] struct.
///
/// # Arguments
/// * `path`: A reference to the path of the image
///
/// # Returns
/// * An [`ImageHash`]-struct that can be encoded into a string representation
/// * An [`ImageHashError`] if something went wrong while loading the image
pub fn difference_hash(path: &Path) -> Result<ImageHash, ImageHashError> {
    // create the hasher
    let hasher = DifferenceHasher::default();

    hasher.hash_from_path(path)
}

/// Calculate the perceptual hash for an image at the specified path. Uses the default
/// width and height of 8 x 8 pixels as well as the default factor of 4.
/// If you want to use something else please directly use the [`PerceptualHasher`] struct.
///
/// # Arguments
/// * `path`: A reference to the path of the image
///
/// # Returns
/// * An [`ImageHash`]-struct that can be encoded into a string representation
/// * An [`ImageHashError`] if something went wrong while loading the image
pub fn perceptual_hash(path: &Path) -> Result<ImageHash, ImageHashError> {
    // create the hasher
    let hasher = PerceptualHasher::default();

    hasher.hash_from_path(path)
}

// public modules
pub mod average;
pub mod difference;
pub mod median;
pub mod perceptual;

// private modules
mod imageops;
mod imghash;
mod math;

// public exports
pub use crate::imageops::ColorSpace;
pub use crate::imghash::ImageHash;
pub use crate::imghash::ImageHashError;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_IMG: &str = "./data/img/test.png";
    const TXT_FILE: &str = "./data/misc/test.txt";

    #[test]
    fn test_average_hash() {
        // Arrange
        let path = Path::new(TEST_IMG);

        // Act
        let hash = average_hash(path);

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), "ffffff0e00000301")
    }

    #[test]
    fn test_average_hash_with_txt_file() {
        // Arrange
        let path = Path::new(TXT_FILE);

        // Act
        let hash = average_hash(path);

        // Assert
        assert!(hash.is_err());
    }

    #[test]
    fn test_median_hash() {
        // Arrange
        let path = Path::new(TEST_IMG);

        // Act
        let hash = median_hash(path);

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), "ffffff1e00000301")
    }

    #[test]
    fn test_median_hash_with_txt_file() {
        // Arrange
        let path = Path::new(TXT_FILE);

        // Act
        let hash = median_hash(path);

        // Assert
        assert!(hash.is_err());
    }

    #[test]
    fn test_difference_hash() {
        // Arrange
        let path = Path::new(TEST_IMG);

        // Act
        let hash = difference_hash(path);

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), "cc99717ed9ea0627")
    }

    #[test]
    fn test_difference_hash_with_txt_file() {
        // Arrange
        let path = Path::new(TXT_FILE);

        // Act
        let hash = difference_hash(path);

        // Assert
        assert!(hash.is_err());
    }

    #[test]
    fn test_perceptual_hash() {
        // Arrange
        let path = Path::new(TEST_IMG);

        // Act
        let hash = perceptual_hash(path);

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), "acdbe86135344e3a")
    }

    #[test]
    fn test_perceptual_hash_with_txt_file() {
        // Arrange
        let path = Path::new(TXT_FILE);

        // Act
        let hash = perceptual_hash(path);

        // Assert
        assert!(hash.is_err());
    }
}
