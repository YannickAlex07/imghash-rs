use crate::{
    imageops::convert,
    math::{dct2_over_matrix_in_place, median, Axis},
    ColorSpace, ImageHash, ImageHashError, ImageHasher,
};

#[derive(Debug, Clone)]
pub struct PerceptualHasher {
    /// The target width of the matrix
    width: u8,

    /// The target height of the matrix
    height: u8,

    /// The factor for the DCT matrix. We will rescale the image to
    /// (width * factor, height * factor) before we calculate the DCT on it.
    factor: u8,

    /// The color space which will be used for grayscaling.
    /// Default is Rec. 601
    color_space: ColorSpace,
}

impl PerceptualHasher {
    pub fn new(
        width: u8,
        height: u8,
        factor: u8,
        color_space: ColorSpace,
    ) -> Result<Self, ImageHashError> {
        if width == 0 || height == 0 || factor == 0 {
            return Err(ImageHashError::EmptyMatrix);
        }

        Ok(Self {
            width,
            height,
            factor,
            color_space,
        })
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn factor(&self) -> u8 {
        self.factor
    }

    pub fn color_space(&self) -> ColorSpace {
        self.color_space
    }
}

impl ImageHasher for PerceptualHasher {
    fn hash_from_img(&self, img: &image::DynamicImage) -> Result<ImageHash, ImageHashError> {
        if self.width == 0 || self.height == 0 {
            return Err(ImageHashError::EmptyMatrix);
        }

        let width = self.width as u32 * self.factor as u32;
        let height = self.height as u32 * self.factor as u32;

        let high_freq = convert(img, width, height, self.color_space);

        // convert the higher frequency image to a matrix of f64
        let mut dct_matrix = high_freq
            .as_bytes()
            .iter()
            .copied()
            .map(|v| v as f64)
            .collect::<Vec<_>>();

        // now we compute the DCT for each column and then for each row
        dct2_over_matrix_in_place(&mut dct_matrix, width as usize, Axis::Column);
        dct2_over_matrix_in_place(&mut dct_matrix, width as usize, Axis::Row);

        // now we crop the dct matrix to the actual target width and height
        let scaled_matrix = dct_matrix
            .chunks(width as usize)
            .take(self.height as usize)
            .flat_map(|row| &row[0..self.width as usize])
            .copied()
            .collect::<Vec<_>>();

        // compute the median over the flattened matrix
        let median = median(scaled_matrix.iter().copied()).ok_or(ImageHashError::EmptyMatrix)?;

        ImageHash::from_bool_iter(
            scaled_matrix.into_iter().map(|pixel| pixel > median),
            self.width,
            self.height,
        )
    }
}

impl Default for PerceptualHasher {
    fn default() -> Self {
        PerceptualHasher {
            width: 8,
            height: 8,
            factor: 4,
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

    const REC_601_HASH: &str = "acdbe86135344e3a";
    const REC_709_HASH: &str = "acdbe86135344e3a";

    #[test]
    fn test_new_with_zero_width() {
        let result = PerceptualHasher::new(0, 8, 4, ColorSpace::REC601);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_zero_height() {
        let result = PerceptualHasher::new(8, 0, 4, ColorSpace::REC601);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_zero_factor() {
        let result = PerceptualHasher::new(8, 8, 0, ColorSpace::REC601);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_valid_dimensions() {
        let result = PerceptualHasher::new(8, 8, 4, ColorSpace::REC601);
        assert!(result.is_ok());
        let hasher = result.unwrap();
        assert_eq!(hasher.width(), 8);
        assert_eq!(hasher.height(), 8);
        assert_eq!(hasher.factor(), 4);
        assert_eq!(hasher.color_space(), ColorSpace::REC601);
    }

    #[test]
    fn test_perceptual_hash_from_img() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = PerceptualHasher::default();

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), REC_601_HASH)
    }

    #[test]
    fn test_perceptual_hash_from_img_with_rec_709() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = PerceptualHasher::new(8, 8, 4, ColorSpace::REC709).unwrap();

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), REC_709_HASH)
    }

    #[test]
    fn test_perceptual_hash_from_path() {
        // Arrange
        let hasher = PerceptualHasher::default();

        // Act
        let hash = hasher.hash_from_path(Path::new(TEST_IMG));

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), REC_601_HASH)
    }

    #[test]
    fn test_perceptual_hash_from_img_with_non_default_size() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = PerceptualHasher::new(16, 16, 4, ColorSpace::REC601).unwrap();

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert!(hash.is_ok());
        let hash = hash.unwrap();
        assert_eq!(hash.shape(), (16, 16));
    }

    #[test]
    fn test_perceptual_hash_from_nonexisting_path() {
        // Arrange
        let hasher = PerceptualHasher::default();

        // Act
        let hash = hasher.hash_from_path(Path::new("./does/not/exist.png"));

        // Assert
        assert!(hash.is_err());
    }

    #[test]
    fn test_perceptual_hash_from_txt_file() {
        // Arrange
        let hasher = PerceptualHasher::default();

        // Act
        let hash = hasher.hash_from_path(Path::new(TXT_FILE));

        // Assert
        assert!(hash.is_err());
    }
}
