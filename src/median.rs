use crate::{imageops::convert, ColorSpace, ImageHash, ImageHashError, ImageHasher};

#[derive(Debug, Clone)]
pub struct MedianHasher {
    /// The target width of the matrix
    width: u32,

    /// The target height of the matrix
    height: u32,

    /// The color space which will be used for grayscaling.
    /// Default is Rec. 601
    color_space: ColorSpace,
}

impl MedianHasher {
    pub fn new(width: u32, height: u32, color_space: ColorSpace) -> Result<Self, ImageHashError> {
        if width == 0 || height == 0 {
            return Err(ImageHashError::EmptyMatrix);
        }

        Ok(Self {
            width,
            height,
            color_space,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn color_space(&self) -> ColorSpace {
        self.color_space
    }
}

impl ImageHasher for MedianHasher {
    fn hash_from_img(&self, img: &image::DynamicImage) -> Result<ImageHash, ImageHashError> {
        if self.width == 0 || self.height == 0 {
            return Err(ImageHashError::EmptyMatrix);
        }

        let converted = convert(img, self.width, self.height, self.color_space);

        let mut values: Vec<u8> = converted.as_bytes().to_vec();

        let len = values.len();
        let median = *values.select_nth_unstable(len / 2).1;

        ImageHash::from_bool_iter(
            converted.as_bytes().iter().map(|&p| p > median),
            self.width,
            self.height,
        )
    }
}

impl Default for MedianHasher {
    fn default() -> Self {
        MedianHasher {
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

    const REC_601_HASH: &str = "ffffff1e00000301";
    const REC_709_HASH: &str = "ffffff1e00000301";

    #[test]
    fn test_new_with_zero_width() {
        let result = MedianHasher::new(0, 8, ColorSpace::REC601);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_zero_height() {
        let result = MedianHasher::new(8, 0, ColorSpace::REC601);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_valid_dimensions() {
        let result = MedianHasher::new(8, 8, ColorSpace::REC601);
        assert!(result.is_ok());
        let hasher = result.unwrap();
        assert_eq!(hasher.width(), 8);
        assert_eq!(hasher.height(), 8);
        assert_eq!(hasher.color_space(), ColorSpace::REC601);
    }

    #[test]
    fn test_median_hash_from_img() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = MedianHasher::default();

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), REC_601_HASH)
    }

    #[test]
    fn test_median_hash_from_img_with_rec_709() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = MedianHasher::new(8, 8, ColorSpace::REC709).unwrap();

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), REC_709_HASH)
    }

    #[test]
    fn test_median_hash_from_path() {
        // Arrange
        let hasher = MedianHasher::default();

        // Act
        let hash = hasher.hash_from_path(Path::new(TEST_IMG));

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), REC_601_HASH)
    }

    #[test]
    fn test_median_hash_from_img_with_non_default_size() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = MedianHasher::new(16, 16, ColorSpace::REC601).unwrap();

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert!(hash.is_ok());
        let hash = hash.unwrap();
        assert_eq!(hash.shape(), (16, 16));
    }

    #[test]
    fn test_median_hash_from_nonexisting_path() {
        // Arrange
        let hasher = MedianHasher::default();

        // Act
        let hash = hasher.hash_from_path(Path::new("./does/not/exist.png"));

        // Assert
        assert!(hash.is_err());
    }

    #[test]
    fn test_median_hash_from_txt_file() {
        // Arrange
        let hasher = MedianHasher::default();

        // Act
        let hash = hasher.hash_from_path(Path::new(TXT_FILE));

        // Assert
        assert!(hash.is_err());
    }
}
