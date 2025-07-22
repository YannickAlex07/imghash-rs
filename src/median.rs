use crate::{imageops::ImageOps, ColorSpace, ImageHash, ImageHasher};

pub struct MedianHasher {
    /// The target width of the matrix
    pub width: u32,

    /// The target height of the matrix
    pub height: u32,

    /// The color space which will be used for grayscaling.
    /// Default is Rec. 601
    pub color_space: ColorSpace,
}

impl ImageHasher for MedianHasher {
    fn hash_from_img(&self, img: &image::DynamicImage) -> ImageHash {
        let converted = self.convert(img, self.width, self.height, self.color_space);

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

impl ImageOps for MedianHasher {}

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
    fn test_median_hash_from_img() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher: MedianHasher = MedianHasher {
            ..Default::default()
        };

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert_eq!(hash.encode(), REC_601_HASH)
    }

    #[test]
    fn test_median_hash_from_img_with_rec_709() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = MedianHasher {
            color_space: ColorSpace::REC709,
            ..Default::default()
        };

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert_eq!(hash.encode(), REC_709_HASH)
    }

    #[test]
    fn test_median_hash_from_path() {
        // Arrange
        let hasher = MedianHasher {
            ..Default::default()
        };

        // Act
        let hash = hasher.hash_from_path(Path::new(TEST_IMG));

        // Assert
        match hash {
            Ok(hash) => assert_eq!(hash.encode(), REC_601_HASH),
            Err(err) => panic!("could not read image: {:?}", err),
        }
    }

    #[test]
    fn test_median_hash_from_nonexisting_path() {
        // Arrange
        let hasher = MedianHasher {
            ..Default::default()
        };

        // Act
        let hash = hasher.hash_from_path(Path::new("./does/not/exist.png"));

        // Assert
        match hash {
            Ok(hash) => panic!("found hash for non-existing image: {:?}", hash),
            Err(_) => (),
        }
    }

    #[test]
    fn test_median_hash_from_txt_file() {
        // Arrange
        let hasher = MedianHasher {
            ..Default::default()
        };

        // Act
        let hash = hasher.hash_from_path(Path::new(TXT_FILE));

        // Assert
        match hash {
            Ok(hash) => panic!("found hash for non-existing image: {:?}", hash),
            Err(_) => (),
        }
    }
}
