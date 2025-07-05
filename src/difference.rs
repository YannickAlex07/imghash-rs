use crate::{imageops::ImageOps, ColorSpace, ImageHash, ImageHasher};

pub struct DifferenceHasher {
    /// The target width of the matrix
    pub width: u32,

    /// The target height of the matrix
    pub height: u32,

    pub color_space: ColorSpace,
}

impl ImageHasher for DifferenceHasher {
    fn hash_from_img(&self, img: &image::DynamicImage) -> ImageHash {
        let converted = self.convert(img, self.width + 1, self.height, self.color_space);

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

impl ImageOps for DifferenceHasher {}

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
    fn test_difference_hash_from_img() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = DifferenceHasher {
            ..Default::default()
        };

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert_eq!(hash.encode(), REC_601_HASH)
    }

    #[test]
    fn test_difference_hash_from_img_with_rec709() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = DifferenceHasher {
            color_space: ColorSpace::REC709,
            ..Default::default()
        };

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert_eq!(hash.encode(), REC_709_HASH)
    }

    #[test]
    fn test_difference_hash_from_path() {
        // Arrange
        let hasher = DifferenceHasher {
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
    fn test_difference_hash_from_nonexisting_path() {
        // Arrange
        let hasher = DifferenceHasher {
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
    fn test_difference_hash_from_txt_file() {
        // Arrange
        let hasher = DifferenceHasher {
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
