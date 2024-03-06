use crate::{convert::Convert, ImageHash, ImageHasher};

pub struct AverageHasher {
    /// The target width of the matrix
    pub width: u32,

    /// The target height of the matrix
    pub height: u32,
}

impl ImageHasher for AverageHasher {
    fn hash_from_img(&self, img: &image::DynamicImage) -> ImageHash {
        let converted = self.convert(img, self.width, self.height);
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

        ImageHash { matrix }
    }
}

impl Default for AverageHasher {
    fn default() -> Self {
        AverageHasher {
            width: 8,
            height: 8,
        }
    }
}

impl Convert for AverageHasher {}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use image::io::Reader as ImageReader;

    use super::*;

    const TEST_IMG: &str = "./data/img/test.png";
    const TXT_FILE: &str = "./data/misc/test.txt";

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
        assert_eq!(hash.encode(), "ffffff0e00000301")
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
            Ok(hash) => assert_eq!(hash.encode(), "ffffff0e00000301"),
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
