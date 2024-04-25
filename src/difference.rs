use crate::{convert::Convert, ColorSpace, ImageHash, ImageHasher};

pub struct DifferenceHasher {
    /// The target width of the matrix
    pub width: u32,

    /// The target height of the matrix
    pub height: u32,
}

impl ImageHasher for DifferenceHasher {
    fn hash_from_img(&self, img: &image::DynamicImage) -> ImageHash {
        let converted = self.convert(img, self.width + 1, self.height, ColorSpace::REC601);

        // we will compute the differences on this matrix
        let compare_matrix: Vec<Vec<u8>> = converted
            .as_bytes()
            .to_vec()
            .chunks((self.width + 1) as usize)
            .map(|x| x.to_vec())
            .collect();

        // the results are stored in this matrix
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
    use std::path::Path;

    use image::io::Reader as ImageReader;

    use super::*;

    const TEST_IMG: &str = "./data/img/test.png";
    const TXT_FILE: &str = "./data/misc/test.txt";

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
        assert_eq!(hash.encode(), "c49b397ed9ea0627")
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
            Ok(hash) => assert_eq!(hash.encode(), "c49b397ed9ea0627"),
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
