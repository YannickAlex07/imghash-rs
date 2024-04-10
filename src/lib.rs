use average::AverageHasher;
use difference::DifferenceHasher;
use image::ImageError;
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

/// Calculate the average hash for an image at the specified path. Uses the default
/// width and height of 8 x 8 pixels. If you want to use something else please directly use
/// the [`AverageHasher`] struct.
///
/// # Arguments
/// * `path`: A reference to the path of the image
///
/// # Returns
/// * An [`ImageHash`]-struct that can be encoded into a string representation
/// * An [`ImageError`] if something went wrong while loading the image
pub fn average_hash(path: &Path) -> Result<ImageHash, ImageError> {
    // create the hasher
    let hasher = AverageHasher {
        ..Default::default()
    };

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
/// * An [`ImageError`] if something went wrong while loading the image
pub fn difference_hash(path: &Path) -> Result<ImageHash, ImageError> {
    // create the hasher
    let hasher = DifferenceHasher {
        ..Default::default()
    };

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
/// * An [`ImageError`] if something went wrong while loading the image
pub fn perceptual_hash(path: &Path) -> Result<ImageHash, ImageError> {
    // create the hasher
    let hasher = PerceptualHasher {
        ..Default::default()
    };

    hasher.hash_from_path(path)
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
        assert_eq!(hash.unwrap().encode(), "ffffff0e00000301")
    }

    #[test]
    fn test_average_hash_with_txt_file() {
        // Arrange
        let path = Path::new(TXT_FILE);

        // Act
        let hash = average_hash(path);

        // Assert
        match hash {
            Ok(_) => panic!("should not be able to calculate hash for txt file"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_difference_hash() {
        // Arrange
        let path = Path::new(TEST_IMG);

        // Act
        let hash = difference_hash(path);

        // Assert
        assert_eq!(hash.unwrap().encode(), "c49b397ed9ea0627")
    }

    #[test]
    fn test_difference_hash_with_txt_file() {
        // Arrange
        let path = Path::new(TXT_FILE);

        // Act
        let hash = difference_hash(path);

        // Assert
        match hash {
            Ok(_) => panic!("should not be able to calculate hash for txt file"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_perceptual_hash() {
        // Arrange
        let path = Path::new(TEST_IMG);

        // Act
        let hash = perceptual_hash(path);

        // Assert
        assert_eq!(hash.unwrap().encode(), "acdbe86135344e3a")
    }

    #[test]
    fn test_perceptual_hash_with_txt_file() {
        // Arrange
        let path = Path::new(TXT_FILE);

        // Act
        let hash = perceptual_hash(path);

        // Assert
        match hash {
            Ok(_) => panic!("should not be able to calculate hash for txt file"),
            Err(_) => assert!(true),
        }
    }
}
