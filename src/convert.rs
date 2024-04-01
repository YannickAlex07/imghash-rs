use image::{imageops::FilterType, DynamicImage};

pub trait Convert {
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
    fn convert(&self, img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
        let filter = FilterType::Lanczos3;
        img.grayscale().resize_exact(width, height, filter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use image::io::Reader as ImageReader;
    use std::path::Path;

    pub struct Converter;
    impl Convert for Converter {}

    #[test]
    fn test_convert() {
        // Arrange
        let test_img = ImageReader::open(Path::new("./data/img/test.png"))
            .unwrap()
            .decode()
            .unwrap();

        let converted_img = ImageReader::open(Path::new("./data/img/convert.png"))
            .unwrap()
            .decode()
            .unwrap();

        let converter = Converter {};

        // Act
        let converted = converter.convert(&test_img, 32, 32);

        // Assert
        assert_eq!(converted, converted_img);
    }
}
