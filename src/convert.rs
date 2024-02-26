use image::{imageops::FilterType, DynamicImage};

pub trait Convert {
    fn convert(&self, img: DynamicImage, width: u32, height: u32) -> DynamicImage {
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
    fn test_resize() {
        let img = ImageReader::open(Path::new("./data/example.jpg"))
            .unwrap()
            .decode()
            .unwrap();

        // let resizer = Resizer {};
        // let resized = resizer.resize(img, 500, 500, ResizeMode::Stretch);

        let resized = img.grayscale().resize_exact(8, 8, FilterType::Lanczos3);

        resized
            .save_with_format(Path::new("./data/resized.png"), image::ImageFormat::Png)
            .expect("Failed to save resized image");
    }
}
