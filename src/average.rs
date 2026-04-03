use crate::{imageops::convert, ColorSpace, ImageHash, ImageHashError, ImageHasher};

#[derive(Debug, Clone)]
pub struct AverageHasher {
    /// The target width of the matrix
    pub width: u32,

    /// The target height of the matrix
    pub height: u32,

    /// The color space which will be used for grayscaling.
    /// Default is Rec. 601
    pub color_space: ColorSpace,
}

impl ImageHasher for AverageHasher {
    fn hash_from_img(&self, img: &image::DynamicImage) -> Result<ImageHash, ImageHashError> {
        if self.width == 0 || self.height == 0 {
            return Err(ImageHashError::EmptyMatrix);
        }

        let converted = convert(img, self.width, self.height, self.color_space);
        let mean = converted
            .as_bytes()
            .iter()
            .fold(0, |acc, x| acc + *x as usize)
            / (self.width as usize * self.height as usize);

        ImageHash::from_bool_iter(
            converted.as_bytes().iter().map(|&p| p as usize > mean),
            self.width,
            self.height,
        )
    }
}

impl Default for AverageHasher {
    fn default() -> Self {
        AverageHasher {
            width: 8,
            height: 8,
            color_space: ColorSpace::REC601,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use image::ImageReader;

    use super::*;

    const TEST_IMG: &str = "./data/img/test.png";
    const TXT_FILE: &str = "./data/misc/test.txt";

    const REC_601_HASH: &str = "ffffff0e00000301";
    const REC_709_HASH: &str = "ffffff0e00000301";

    #[test]
    fn test_average_hash_from_img() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = AverageHasher {
            ..Default::default()
        };

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), REC_601_HASH)
    }

    #[test]
    fn test_average_hash_from_img_with_rec_709() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = AverageHasher {
            color_space: ColorSpace::REC709,
            ..Default::default()
        };

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), REC_709_HASH)
    }

    #[test]
    fn test_average_hash_from_path() {
        // Arrange
        let hasher = AverageHasher {
            ..Default::default()
        };

        // Act
        let hash = hasher.hash_from_path(Path::new(TEST_IMG));

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), REC_601_HASH)
    }

    #[test]
    fn test_average_hash_from_nonexisting_path() {
        // Arrange
        let hasher = AverageHasher {
            ..Default::default()
        };

        // Act
        let hash = hasher.hash_from_path(Path::new("./does/not/exist.png"));

        // Assert
        assert!(hash.is_err());
    }

    #[test]
    fn test_average_hash_from_txt_file() {
        // Arrange
        let hasher = AverageHasher {
            ..Default::default()
        };

        // Act
        let hash = hasher.hash_from_path(Path::new(TXT_FILE));

        // Assert
        assert!(hash.is_err());
    }
}
