use image::{imageops::FilterType, DynamicImage};

pub trait Convert {
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
        let test_img = ImageReader::open(Path::new("./data/convert/test.png"))
            .unwrap()
            .decode()
            .unwrap();

        let converted_img = ImageReader::open(Path::new("./data/convert/converted.png"))
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
