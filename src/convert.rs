use image::{imageops::FilterType, DynamicImage, GenericImageView, GrayImage};
use rayon::prelude::*;

pub enum ColorSpace {
    REC709,
    REC601,
}

pub trait Convert {
    fn grayscale(&self, img: &DynamicImage, space: ColorSpace) -> DynamicImage {
        let mut buffer = GrayImage::new(img.width(), img.height());

        let coefficients: [f32; 3];
        match space {
            ColorSpace::REC709 => coefficients = [0.2126, 0.7152, 0.0722],
            ColorSpace::REC601 => coefficients = [0.299, 0.587, 0.114],
        }

        buffer.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let [r, g, b, _] = img.get_pixel(x, y).0;

            let luma = (coefficients[0] * r as f32
                + coefficients[1] * g as f32
                + coefficients[2] * b as f32) as u8;

            *pixel = image::Luma([luma]);
        });

        DynamicImage::ImageLuma8(buffer)
    }

    /// Converts a given [`DynamicImage`] by conveting it to grayscale and then resizing it
    /// to the specified size.
    ///
    /// # Arguments
    /// * `img`: A reference to the image to convert
    /// * `width`: The final width of the rescaled image
    /// * `height`: The final height of the rescaled image
    ///
    /// # Returns
    /// * The converted dynamic image
    fn convert(
        &self,
        img: &DynamicImage,
        width: u32,
        height: u32,
        color_space: ColorSpace,
    ) -> DynamicImage {
        let filter = FilterType::Lanczos3;

        let grayscale_img = self.grayscale(img, color_space);
        grayscale_img.resize_exact(width, height, filter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use image::io::Reader as ImageReader;
    use std::path::Path;

    pub struct Converter;
    impl Convert for Converter {}

    const TEST_IMG: &str = "./data/img/test.png";

    const REC_601_IMG: &str = "./data/img/gray-601.png";
    const REC_709_IMG: &str = "./data/img/gray-709.png";

    const REC_601_SCALED_IMG: &str = "./data/img/gray-scaled-601.png";
    const REC_709_SCALED_IMG: &str = "./data/img/gray-scaled-709.png";

    #[test]
    fn test_grayscale_with_601() {
        // Arrange
        let test_img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let grayscale_img = ImageReader::open(Path::new(REC_601_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let converter = Converter {};

        // Act
        let grayscale = converter.grayscale(&test_img, ColorSpace::REC601);

        // Assert
        assert_eq!(grayscale, grayscale_img);
    }

    #[test]
    fn test_grayscale_with_709() {
        // Arrange
        let test_img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let grayscale_img = ImageReader::open(Path::new(REC_709_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let converter = Converter {};

        // Act
        let grayscale = converter.grayscale(&test_img, ColorSpace::REC709);

        // Assert
        assert_eq!(grayscale, grayscale_img);
    }

    #[test]
    fn test_convert_with_rec_709() {
        // Arrange
        let test_img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let converted_img = ImageReader::open(Path::new(REC_709_SCALED_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let converter = Converter {};

        // Act
        let converted = converter.convert(&test_img, 32, 32, ColorSpace::REC709);

        // Assert
        assert_eq!(converted, converted_img);
    }

    #[test]
    fn test_convert_with_rec_601() {
        // Arrange
        let test_img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let converted_img = ImageReader::open(Path::new(REC_601_SCALED_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let converter = Converter {};

        // Act
        let converted = converter.convert(&test_img, 32, 32, ColorSpace::REC601);

        // Assert
        assert_eq!(converted, converted_img);
    }
}
