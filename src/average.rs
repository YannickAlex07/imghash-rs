use crate::{imageops::ImageOps, ColorSpace, ImageHash, ImageHasher};

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
    fn hash_from_img(&self, img: &image::DynamicImage) -> ImageHash {
        let converted = self.convert(img, self.width, self.height, &self.color_space);
        let mean: usize = converted
            .as_bytes()
            .to_vec()
            .iter()
            .fold(0, |acc, x| acc + *x as usize)
            / (self.width * self.height) as usize;

        let mut bits = vec![false; (self.width * self.height) as usize];
        for (i, p) in converted.as_bytes().to_vec().iter().enumerate() {
            if *p as usize > mean {
                bits[i] = true;
            }
        }

        let matrix = bits
            .chunks(self.width as usize)
            .map(|x| x.to_vec())
            .collect();

        ImageHash::new(matrix)
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

impl ImageOps for AverageHasher {}

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
        assert_eq!(hash.encode(), REC_601_HASH)
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
        assert_eq!(hash.encode(), REC_709_HASH)
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
        match hash {
            Ok(hash) => assert_eq!(hash.encode(), REC_601_HASH),
            Err(err) => panic!("could not read image: {:?}", err),
        }
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
        match hash {
            Ok(hash) => panic!("found hash for non-existing image: {:?}", hash),
            Err(_) => (),
        }
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
        match hash {
            Ok(hash) => panic!("found hash for non-existing image: {:?}", hash),
            Err(_) => (),
        }
    }
}
