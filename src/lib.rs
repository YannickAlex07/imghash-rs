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

/// Calculate the average hash for an image at the specified path
///
/// # Arguments
/// * `path`: A reference to the path of the image
/// * `width`: The width of the final matrix which will be encoded into the hash
/// * `height`: The height of the final matrix which will be encoded into the hash
///
/// # Returns
/// * An [ImageHash]-struct that can be encoded into a string representation
/// * An [ImageError] if something went wrong while loading the image
pub fn average_hash(path: &Path, width: u32, height: u32) -> Result<ImageHash, ImageError> {
    // create the hasher
    let hasher = AverageHasher { width, height };
    hasher.hash_from_path(path)
}

/// Calculate the difference hash for an image at the specified path
///
/// # Arguments
/// * `path`: A reference to the path of the image
/// * `width`: The width of the final matrix which will be encoded into the hash
/// * `height`: The height of the final matrix which will be encoded into the hash
///
/// # Returns
/// * An [ImageHash]-struct that can be encoded into a string representation
/// * An [ImageError] if something went wrong while loading the image
pub fn difference_hash(path: &Path, width: u32, height: u32) -> Result<ImageHash, ImageError> {
    // create the hasher
    let hasher = DifferenceHasher { width, height };
    hasher.hash_from_path(path)
}

/// Calculate the perceptual hash for an image at the specified path
///
/// # Arguments
/// * `path`: A reference to the path of the image
/// * `width`: The width of the final matrix which will be encoded into the hash
/// * `height`: The height of the final matrix which will be encoded into the hash
/// * `factor`: The factor by which the input image will be scaled for calculating the DCT
///
/// # Returns
/// * An [ImageHash]-struct that can be encoded into a string representation
/// * An [ImageError] if something went wrong while loading the image
pub fn perceptual_hash(
    path: &Path,
    width: u32,
    height: u32,
    factor: u32,
) -> Result<ImageHash, ImageError> {
    // create the hasher
    let hasher = PerceptualHasher {
        width,
        height,
        factor,
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
        let hash = average_hash(path, 8, 8);

        // Assert
        assert_eq!(hash.unwrap().encode(), "ffffff0e00000301")
    }

    #[test]
    fn test_average_hash_with_txt_file() {
        // Arrange
        let path = Path::new(TXT_FILE);

        // Act
        let hash = average_hash(path, 8, 8);

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
        let hash = difference_hash(path, 8, 8);

        // Assert
        assert_eq!(hash.unwrap().encode(), "c49b397ed9ea0627")
    }

    #[test]
    fn test_difference_hash_with_txt_file() {
        // Arrange
        let path = Path::new(TXT_FILE);

        // Act
        let hash = difference_hash(path, 8, 8);

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
        let hash = perceptual_hash(path, 8, 8, 4);

        // Assert
        assert_eq!(hash.unwrap().encode(), "157d1d1b193c7c1c")
    }

    #[test]
    fn test_perceptual_hash_with_txt_file() {
        // Arrange
        let path = Path::new(TXT_FILE);

        // Act
        let hash = perceptual_hash(path, 8, 8, 4);

        // Assert
        match hash {
            Ok(_) => panic!("should not be able to calculate hash for txt file"),
            Err(_) => assert!(true),
        }
    }
}
