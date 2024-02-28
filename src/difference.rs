use std::path::Path;

use image::ImageError;

use crate::{convert::Convert, ImageHash, ImageHasher};

pub struct DifferenceHasher {
    pub width: u32,
    pub height: u32,
}

impl ImageHasher for DifferenceHasher {
    fn hash_from_path(&self, path: &Path) -> Result<ImageHash, ImageError> {
        match image::io::Reader::open(path)?.decode() {
            Ok(img) => Ok(self.hash_from_img(&img)),
            Err(e) => Err(e),
        }
    }

    fn hash_from_img(&self, img: &image::DynamicImage) -> ImageHash {
        let converted = self.convert(img, self.width + 1, self.height);
        let compare_matrix: Vec<Vec<u8>> = converted
            .as_bytes()
            .to_vec()
            .chunks((self.width + 1) as usize)
            .map(|x| x.to_vec())
            .collect();

        let mut matrix: Vec<Vec<bool>> = vec![];
        for row in &compare_matrix {
            let r: Vec<bool> = row.windows(2).map(|window| window[0] < window[1]).collect();
            matrix.push(r);
        }

        ImageHash { matrix }
    }
}

impl Default for DifferenceHasher {
    fn default() -> Self {
        DifferenceHasher {
            width: 8,
            height: 8,
        }
    }
}

impl Convert for DifferenceHasher {}

#[cfg(test)]
mod tests {
    use image::io::Reader as ImageReader;

    use super::*;

    const TEST_IMG: &str = "./data/test.png";

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
        assert_eq!(hash.python_safe_encode(), "c49b397ed9ea0627")
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
            Ok(hash) => assert_eq!(hash.python_safe_encode(), "c49b397ed9ea0627"),
            Err(_) => assert!(false),
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
            Ok(hash) => assert!(false, "found hash for non-existing image: {:?}", hash),
            Err(_) => assert!(true),
        }
    }
}
