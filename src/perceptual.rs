use crate::{
    imageops::ImageOps,
    math::{dct2_over_matrix_in_place, median, Axis},
    ColorSpace, ImageHash, ImageHasher,
};

pub struct PerceptualHasher {
    /// The target width of the matrix
    pub width: u32,

    /// The target height of the matrix
    pub height: u32,

    /// The factor for the DCT matrix. We will rescale the image to (width * height) * 4
    /// before we calculate the DCT on it.
    pub factor: u32,

    pub color_space: ColorSpace,
}

impl ImageHasher for PerceptualHasher {
    fn hash_from_img(&self, img: &image::DynamicImage) -> ImageHash {
        let width = self.width * self.factor;
        let height = self.height * self.factor;

        let high_freq = self.convert(img, width, height, self.color_space);

        // convert the higher frequency image to a matrix of f64
        let mut dct_matrix = high_freq
            .as_bytes()
            .into_iter()
            .copied()
            .map(|v| v as f64)
            .collect::<Vec<_>>();

        // now we compute the DCT for each column and then for each row
        dct2_over_matrix_in_place(&mut dct_matrix, width as usize, Axis::Column);
        dct2_over_matrix_in_place(&mut dct_matrix, width as usize, Axis::Row);

        // now we crop the dct matrix to the actual target width and height
        let scaled_matrix = dct_matrix
            .chunks(width as usize)
            .take(self.height as usize)
            .flat_map(|row| &row[0..self.width as usize])
            .copied()
            .collect::<Vec<_>>();

        // compute the median over the flattened matrix
        let median = median(scaled_matrix.iter().copied()).unwrap();

        ImageHash::from_bool_iter(
            scaled_matrix.into_iter().map(|pixel| pixel > median),
            self.width,
            self.height,
        )
    }
}

impl Default for PerceptualHasher {
    fn default() -> Self {
        PerceptualHasher {
            width: 8,
            height: 8,
            factor: 4,
            color_space: ColorSpace::REC601,
        }
    }
}

impl ImageOps for PerceptualHasher {}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use image::ImageReader;

    use super::*;

    const TEST_IMG: &str = "./data/img/test.png";
    const TXT_FILE: &str = "./data/misc/test.txt";

    const REC_601_HASH: &str = "acdbe86135344e3a";
    const REC_709_HASH: &str = "acdbe86135344e3a";

    #[test]
    fn test_perceptual_hash_from_img() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = PerceptualHasher {
            ..Default::default()
        };

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert_eq!(hash.encode(), REC_601_HASH)
    }

    #[test]
    fn test_perceptual_hash_from_img_with_rec_709() {
        // Arrange
        let img = ImageReader::open(Path::new(TEST_IMG))
            .unwrap()
            .decode()
            .unwrap();

        let hasher = PerceptualHasher {
            color_space: ColorSpace::REC709,
            ..Default::default()
        };

        // Act
        let hash = hasher.hash_from_img(&img);

        // Assert
        assert_eq!(hash.encode(), REC_709_HASH)
    }

    #[test]
    fn test_perceptual_hash_from_path() {
        // Arrange
        let hasher = PerceptualHasher {
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
    fn test_perceptual_hash_from_nonexisting_path() {
        // Arrange
        let hasher = PerceptualHasher {
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
    fn test_perceptual_hash_from_txt_file() {
        // Arrange
        let hasher = PerceptualHasher {
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
