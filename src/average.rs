use crate::{ImageHash, ImageHasher};
use std::path::Path;

pub struct AverageHasher {
    pub width: usize,
    pub height: usize,
}

impl ImageHasher for AverageHasher {
    fn hash_from_path(&self, path: &Path) -> ImageHash {
        println!("Path: {:?}", path);
        todo!()
    }

    fn hash_from_img(&self, img: &image::DynamicImage) -> ImageHash {
        println!("Image: {:?}", img);
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA_FOLDER: &str = "test.jpg";

    #[test]
    fn test_average_hash_from_path() {
        // Arrange
        let hasher = AverageHasher {
            width: 8,
            height: 8,
        };

        // Act
        let hash = hasher.hash_from_path(Path::new(TEST_DATA_FOLDER));

        // Assert
        assert_eq!(hash.matrix.len(), 8);
        assert_eq!(hash.matrix[0].len(), 8);
    }
}
