use crate::{imageops::convert, ColorSpace, ImageHash, ImageHashError, ImageHasher};

#[derive(Debug, Clone)]
pub struct DifferenceHasher {
    /// The target width of the matrix
    width: u32,

    /// The target height of the matrix
    height: u32,

    /// The color space which will be used for grayscaling.
    /// Default is Rec. 601
    color_space: ColorSpace,
}

impl DifferenceHasher {
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

impl ImageHasher for DifferenceHasher {
    fn hash_from_img(&self, img: &image::DynamicImage) -> Result<ImageHash, ImageHashError> {
        if self.width == 0 || self.height == 0 {
            return Err(ImageHashError::EmptyMatrix);
        }

        let converted = convert(img, self.width + 1, self.height, self.color_space);

        // we will compute the differences on this matrix
        let compare_matrix: Box<[Box<[u8]>]> = converted
            .as_bytes()
            .chunks((self.width + 1) as usize)
            .map(|x| x.to_vec().into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice();

        ImageHash::from_bool_iter(
            compare_matrix
                .iter()
                .flat_map(|row| row.windows(2).map(|window| window[0] < window[1])),
            self.width,
            self.height,
        )
    }
}

impl Default for DifferenceHasher {
    fn default() -> Self {
        DifferenceHasher {
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

    const REC_601_HASH: &str = "cc99717ed9ea0627";
    const REC_709_HASH: &str = "c499717ed9ea0627";

    #[test]
    fn test_new_with_zero_width() {
        let result = DifferenceHasher::new(0, 8, ColorSpace::REC601);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_zero_height() {
        let result = DifferenceHasher::new(8, 0, ColorSpace::REC601);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_valid_dimensions() {
        let result = DifferenceHasher::new(8, 8, ColorSpace::REC601);
        assert!(result.is_ok());
        let hasher = result.unwrap();
        assert_eq!(hasher.width(), 8);
        assert_eq!(hasher.height(), 8);
        assert_eq!(hasher.color_space(), ColorSpace::REC601);
    }

    #[test]
    fn test_difference_hash_from_img() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = DifferenceHasher::default();

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), REC_601_HASH)
    }

    #[test]
    fn test_difference_hash_from_img_with_rec709() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = DifferenceHasher::new(8, 8, ColorSpace::REC709).unwrap();

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), REC_709_HASH)
    }

    #[test]
    fn test_difference_hash_from_path() {
        // Arrange
        let hasher = DifferenceHasher::default();

        // Act
        let hash = hasher.hash_from_path(Path::new(TEST_IMG));

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), REC_601_HASH)
    }

    #[test]
    fn test_difference_hash_from_img_with_non_default_size() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = DifferenceHasher::new(16, 16, ColorSpace::REC601).unwrap();

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert!(hash.is_ok());
        let hash = hash.unwrap();
        assert_eq!(hash.shape(), (16, 16));
    }

    #[test]
    fn test_difference_hash_from_nonexisting_path() {
        // Arrange
        let hasher = DifferenceHasher::default();

        // Act
        let hash = hasher.hash_from_path(Path::new("./does/not/exist.png"));

        // Assert
        assert!(hash.is_err());
    }

    #[test]
    fn test_difference_hash_from_txt_file() {
        // Arrange
        let hasher = DifferenceHasher::default();

        // Act
        let hash = hasher.hash_from_path(Path::new(TXT_FILE));

        // Assert
        assert!(hash.is_err());
    }
}
