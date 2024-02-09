use image::{imageops::FilterType, DynamicImage};

use crate::ResizeMode;

pub trait Resize {
    fn resize(&self, img: DynamicImage, width: u32, height: u32, mode: ResizeMode) -> DynamicImage {
        let filter = FilterType::Lanczos3;

        match mode {
            ResizeMode::Fit => img.resize(width, height, filter),
            ResizeMode::Fill => img.resize_to_fill(width, height, filter),
            ResizeMode::Stretch => img.resize_exact(width, height, filter),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use image::io::Reader as ImageReader;
    use std::path::Path;

    pub struct Resizer;
    impl Resize for Resizer {}

    #[test]
    fn test_resize() {
        let img = ImageReader::open(Path::new("./data/example.jpg"))
            .unwrap()
            .decode()
            .unwrap();

        let resizer = Resizer {};
        let resized = resizer.resize(img, 500, 500, ResizeMode::Stretch);

        resized
            .save_with_format(
                Path::new("./data/example-stretch.jpg"),
                image::ImageFormat::Jpeg,
            )
            .expect("Failed to save resized image");
    }
}
