use crate::{
    imageops::ImageOps,
    math::{dct2_over_matrix, median, Axis},
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
        let high_freq = self.convert(
            img,
            self.width * self.factor,
            self.height * self.factor,
            &self.color_space,
        );

        // convert the higher frequency image to a matrix of f64
        let high_freq_bytes = high_freq.as_bytes().to_vec();
        let high_freq_matrix: Vec<Vec<f64>> = high_freq_bytes
            .chunks((self.width * self.factor) as usize)
            .map(|x| x.iter().map(|x| *x as f64).collect::<Vec<f64>>())
            .collect();

        // now we compute the DCT for each column and then for each row
        let dct_matrix = dct2_over_matrix(
            &dct2_over_matrix(&high_freq_matrix, Axis::Column),
            Axis::Row,
        );

        // now we crop the dct matrix to the actual target width and height
        let scaled_matrix: Vec<Vec<f64>> = dct_matrix
            .iter()
            .take(self.height as usize)
            .map(|row| row.iter().take(self.width as usize).cloned().collect())
            .collect();

        // compute the median over the flattend matrix
        let flattened: Vec<f64> = scaled_matrix.iter().flatten().copied().collect();
        let median = median(&flattened).unwrap();

        // compare each pixel of our scaled image to the mean
        let mut bits = vec![vec![false; self.width as usize]; self.height as usize];
        for (i, row) in scaled_matrix.iter().enumerate() {
            for (j, pixel) in row.iter().enumerate() {
                bits[i][j] = *pixel > median;
            }
        }

        ImageHash::new(bits)
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
