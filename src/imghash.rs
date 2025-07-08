use bitvec::prelude::*;

#[derive(Debug, PartialEq)]
pub struct ImageHash {
    // The internal bit-vector stored in LSB order.
    data: BitBox<u8, Lsb0>,

    // Number of columns.
    width: u32,

    // Number of rows.
    height: u32,
}

impl ImageHash {
    /// Create a new [`ImageHash`] from the specified bit matrix.
    ///
    /// # Arguments
    /// * `matrix`: A 2D `Vec` of `bool` values in rows of columns.
    ///             All rows must be non-empty and have the same length.
    ///
    /// # Returns
    /// * The new [`ImageHash`].
    pub fn new(matrix: Vec<Vec<bool>>) -> ImageHash {
        if matrix.is_empty() || matrix.first().unwrap().is_empty() {
            panic!("Matrix cannot be empty");
        }

        // Ensures that the matrix rows all have the same length.
        // If not, this is a critical issue and likely a bug in the code
        // that creates the hash -> therefore a panic here is appropriate
        let width = matrix.first().unwrap().len();
        let height = matrix.len();

        if matrix.iter().any(|row| row.len() != width) {
            panic!("All rows must have the same length");
        }

        Self::from_bool_iter(
            matrix.into_iter().flat_map(Vec::into_iter),
            width as u32,
            height as u32,
        )
    }

    /// Create a new [`ImageHash`] from the specified bits stream.
    ///
    /// # Arguments
    /// * `iter`: An iterator that yields `bool` values representing the bits of the hash.
    ///           The length of the stream must match `width * height`.
    /// * `width`: Number of columns of the hash.
    /// * `height`: Number of rows of the hash.
    ///
    /// # Returns
    /// * The new [`ImageHash`].
    pub fn from_bool_iter(
        iter: impl IntoIterator<Item = bool>,
        width: u32,
        height: u32,
    ) -> ImageHash {
        let length = width as usize * height as usize;

        let mut data = bitbox![u8, Lsb0; 0; length];
        let mut count = 0;

        iter.into_iter().enumerate().for_each(|(i, bit)| {
            data.set(i, bit);
            count += 1;
        });

        if count != length {
            panic!("Data length does not match the specified width and height");
        }

        data.fill_uninitialized(false);

        ImageHash {
            data,
            width,
            height,
        }
    }

    /// Create an iterator yielding `bool` values over the bits of an [`ImageHash`].
    ///
    /// # Returns
    /// * Bits from the [`ImageHash`], by column then by row.
    pub fn iter_bool(&self) -> impl Iterator<Item = bool> + '_ {
        self.data.iter().by_vals()
    }

    /// Returns a copy of the underlying matrix that represents the [`ImageHash`].
    #[deprecated(
        since = "1.5.0",
        note = "This method is inefficient because it creates a new Vec<Vec<bool>>. Consider using `iter_bool` instead."
    )]
    pub fn matrix(&self) -> Vec<Vec<bool>> {
        self.data
            .chunks(self.width as usize)
            .map(|chunk| chunk.iter().by_vals().collect::<Vec<bool>>())
            .collect()
    }

    /// Flattens the bit matrix that represents the [`ImageHash`] into a single vector.
    #[deprecated(
        since = "1.5.0",
        note = "This method is inefficient because it creates a new Vec<bool>. Consider using `iter_bool` instead."
    )]
    pub fn flatten(&self) -> Vec<bool> {
        self.iter_bool().collect()
    }

    /// The shape of the matrix that represents the [`ImageHash`], in (number of rows, number of columns).
    pub fn shape(&self) -> (usize, usize) {
        (self.height as usize, self.width as usize)
    }

    /// The hamming distance between this hash and the other hash.
    /// The hamming distance is the number of bits that differ between the two hashes.
    pub fn distance(&self, other: &ImageHash) -> Result<usize, String> {
        if self.shape() != other.shape() {
            return Err("Cannot compute distance of hashes with different sizes".to_string());
        }

        Ok(self
            .data
            .iter()
            .zip(other.data.iter())
            .take(self.width as usize * self.height as usize)
            .fold(0, |acc, (a, b)| acc + (a != b) as usize))
    }

    /// Encodes the bit matrix that represents the [`ImageHash`] into a hexadecimal string.
    /// This implementation is strictly compatible with `imagehash` package for Python.
    pub fn encode(&self) -> String {
        use std::io::Write;

        if self.width == 0 && self.height == 0 {
            panic!("Cannot encode an empty matrix")
        }

        let mut result = Vec::new();

        let length = self.width as usize * self.height as usize;
        let size = (length + 7) / 8;
        let padding = (size * 8) - length;
        let nibbles = (length + 3) / 4;
        let odd = nibbles % 2 == 1;

        let mut buffer = BitBox::<u8, Msb0>::from_iter(
            std::iter::repeat_n(false, padding).chain(self.iter_bool()),
        );
        buffer.fill_uninitialized(false);

        for byte in buffer.as_raw_slice().iter() {
            // Skip the leading '0' if the number of nibbles is odd
            if odd && result.is_empty() {
                write!(&mut result, "{:01x}", byte).unwrap();
            } else {
                write!(&mut result, "{:02x}", byte).unwrap();
            }
        }

        String::from_utf8(result).unwrap()
    }

    /// Decodes a hexadecimal string into a bit matrix that represents the [`ImageHash`].
    /// This implementation is strictly compatible with hashes generated by the `imagehash` package
    /// for Python (read on about the width and height parameter).
    ///
    /// The `width` and `height` parameters are used to specify the dimensions of the matrix that the
    /// hash was originally generated from. This is usually 8 x 8 in the original `imagehash` package.
    /// If you have a hash that was generated with the `imagehash` package, check what you specified for
    /// the `hash_size`-parameter when generating the hash. Use this value for the `width` and `height`.
    ///
    /// This implementation actually deviates slightly from the original imagehash package, because
    /// it allows the decoding of hashes that have been generated on non-square matrices. This is because
    /// the original package actually only allows the generation of hashes on square matrices, however this
    /// crate does allow arbitrary dimensions.
    pub fn decode(s: &str, width: u32, height: u32) -> Result<ImageHash, String> {
        // first we validate that the width and height actually make sense with the given string
        let length = width as usize * height as usize;

        // guard against too small values
        if length == 0 {
            return Err("Width or height cannot be 0".to_string());
        }

        // validate that s is a valid string
        if s.len() == 0 {
            return Err("String is empty".to_string());
        }

        // guard against a string that is too short or too long for the specified size
        let size = (length + 7) / 8;
        let nibbles = (length + 3) / 4;
        let padding = (size * 8) - length;

        if s.len() != nibbles {
            return Err("String is too short or too long for the specified size".to_string());
        }

        // Add padding if the number of nibbles is odd
        let mut iter =
            std::iter::repeat_n('0', if nibbles % 2 == 1 { 1 } else { 0 }).chain(s.chars());

        // we create a bit vector of the correct size
        let mut data = Vec::<u8>::with_capacity(size);

        for _ in 0..size {
            let hi = iter
                .next()
                .unwrap()
                .to_digit(16)
                .ok_or_else(|| "invalid digit found in string".to_string())?;

            let lo = iter
                .next()
                .unwrap()
                .to_digit(16)
                .ok_or_else(|| "invalid digit found in string".to_string())?;

            let value = ((hi << 4) + lo) as u8;
            data.push(value);
        }

        let data =
            BitBox::<u8, Lsb0>::from_iter(data.view_bits::<Msb0>()[padding..].iter().by_vals());

        Ok(ImageHash {
            data,
            width,
            height,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // NEW

    #[test]
    fn test_image_hash_new_with_valid_matrix() {
        // Arrange
        let hash = ImageHash::new(vec![vec![false, true], vec![true, false]]);

        // Assert
        assert_eq!(
            hash,
            ImageHash::new(vec![vec![false, true], vec![true, false]],)
        );
    }

    #[test]
    #[should_panic]
    fn test_image_hash_new_with_invalid_matrix() {
        // should panic as the second row is longer than the first one
        let _ = ImageHash::new(vec![vec![false, true], vec![true, false, false]]);
    }

    // MATRIX

    #[test]
    fn test_image_hash_get_matrix() {
        // Arrange
        let hash = ImageHash::new(vec![vec![false, true], vec![true, false]]);

        // Assert
        assert_eq!(hash.matrix(), vec![vec![false, true], vec![true, false]],);
    }

    // FLATTEN

    #[test]
    fn test_image_hash_flatten() {
        // Arrange
        let hash = ImageHash::new(vec![vec![false, true], vec![true, false]]);

        let expected = vec![false, true, true, false];

        // Act
        let flattened = hash.flatten();

        // Assert
        assert_eq!(flattened, expected);
    }

    // SHAPE

    #[test]
    fn test_image_hash_shape() {
        // Arrange
        let hash = ImageHash::new(vec![vec![false, true], vec![true, false]]);

        let expected = (2, 2);

        // Act
        let flattened = hash.shape();

        // Assert
        assert_eq!(flattened, expected);
    }

    // ENCODING

    #[test]
    fn test_image_hash_encoding() {
        // Arrange

        // -> resulting bit str: 0010 0100 1111 0000
        // -> resulting hex str: 24F0
        let hash = ImageHash::new(vec![
            vec![false, false, true, false],
            vec![false, true, false, false],
            vec![true, true, true, true],
            vec![false, false, false, false],
        ]);

        // Assert
        assert_eq!(hash.encode(), "24f0");
    }

    #[test]
    fn test_image_hash_encoding_with_non_square_matrix() {
        // Arrange

        // -> resulting bit str: 0110 1010 0011 1110 0001
        // -> resulting hex str: 6A3E1
        let hash = ImageHash::new(vec![
            vec![false, true, true, false, true],
            vec![false, true, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, true],
        ]);

        // Assert
        assert_eq!(hash.encode(), "6a3e1");
    }

    #[test]
    fn test_image_hash_encoding_with_uneven_total_bits() {
        // Arrange

        // due to the uneven number of bits, the entire bit string gets padded until
        // it is divisible by 4
        // -> resulting bit str: 0011 0101 0001 1111
        // -> resulting hex str: 351F
        let hash = ImageHash::new(vec![
            vec![false, true, true, false, true],
            vec![false, true, false, false, false],
            vec![true, true, true, true, true],
        ]);

        // Assert
        assert_eq!(hash.encode(), "351f");
    }

    #[test]
    #[should_panic(expected = "Matrix cannot be empty")]
    fn test_image_hash_encoding_with_empty_matrix() {
        let _ = ImageHash::new(vec![]); // <- should panic
    }

    #[test]
    fn test_image_hash_python_safe_encoding_with_single_bit() {
        // Arrange

        // should equal to 1 due to added padding
        // -> resulting bit str: 0001
        // -> resulting hex str: 1
        let hash = ImageHash::new(vec![vec![true]]);

        // Assert
        assert_eq!(hash.encode(), "1");
    }

    // DECODING

    #[test]
    fn test_image_hash_decoding() {
        // Arrange
        let expected = vec![
            vec![false, false, true, false],
            vec![false, true, false, false],
            vec![true, true, true, true],
            vec![false, false, false, false],
        ];

        // Act
        let decoded = ImageHash::decode("24f0", 4, 4).unwrap();

        // Assert
        assert_eq!(decoded.matrix(), expected);
    }

    #[test]
    fn test_image_hash_decoding_with_non_square_matrix() {
        // Arrange
        let expected = vec![
            vec![false, true, true, false, true],
            vec![false, true, false, false, false],
            vec![true, true, true, true, true],
            vec![false, false, false, false, true],
        ];

        // Act
        let decoded = ImageHash::decode("6a3e1", 5, 4).unwrap();

        // Assert
        assert_eq!(decoded.matrix(), expected);
    }

    #[test]
    fn test_image_hash_decoding_with_uneven_total_bits() {
        // Arrange
        let expected = vec![
            vec![false, true, true, false, true],
            vec![false, true, false, false, false],
            vec![true, true, true, true, true],
        ];

        // Act
        let decoded = ImageHash::decode("351f", 5, 3).unwrap();

        // Assert
        assert_eq!(decoded.matrix(), expected);
    }

    #[test]
    fn test_image_hash_decoding_with_single_bit() {
        // Arrange
        let expected = vec![vec![true]];

        // Act
        let decoded = ImageHash::decode("1", 1, 1).unwrap();

        // Assert
        assert_eq!(decoded.matrix(), expected);
    }

    #[test]
    fn test_image_hash_decoding_with_too_short_string() {
        // Act
        let decoded = ImageHash::decode("AB", 2, 5);

        // Assert
        match decoded {
            Ok(_) => panic!("Should not have decoded"),
            Err(e) => assert_eq!(e, "String is too short or too long for the specified size"),
        }
    }

    #[test]
    fn test_image_hash_decoding_with_too_long_string() {
        // Act
        let decoded = ImageHash::decode("ABCD", 2, 2);

        // Assert
        match decoded {
            Ok(_) => panic!("Should not have decoded"),
            Err(e) => assert_eq!(e, "String is too short or too long for the specified size"),
        }
    }

    #[test]
    fn test_image_hash_decoding_with_invalid_string() {
        // Act
        let decoded = ImageHash::decode("!", 2, 2);

        // Assert
        match decoded {
            Ok(_) => panic!("Should not have decoded"),
            Err(e) => assert_eq!(e, "invalid digit found in string"),
        }
    }

    #[test]
    fn test_image_hash_decoding_with_zero_size_matrix() {
        // Act
        let decoded = ImageHash::decode("!", 2, 0);

        // Assert
        match decoded {
            Ok(_) => panic!("Should not have decoded"),
            Err(e) => assert_eq!(e, "Width or height cannot be 0"),
        }
    }

    #[test]
    fn test_image_hash_decoding_with_empty_string() {
        // Act
        let decoded = ImageHash::decode("", 2, 2);

        // Assert
        match decoded {
            Ok(_) => panic!("Should not have decoded"),
            Err(e) => assert_eq!(e, "String is empty"),
        }
    }

    // DISTANCE

    #[test]
    fn test_image_hash_distance_with_unequal_hashes() {
        // Arrange
        let hash1 = ImageHash::new(vec![vec![false, true], vec![true, false]]);

        let hash2 = ImageHash::new(vec![vec![true, true], vec![false, false]]);

        // Act
        let distance = hash1.distance(&hash2);

        // Assert
        match distance {
            Ok(d) => assert_eq!(d, 2),
            Err(_) => panic!("Should not have errored"),
        }
    }

    #[test]
    fn test_image_hash_distance_with_equal_hashes() {
        // Arrange
        let hash1 = ImageHash::new(vec![vec![false, true], vec![true, false]]);

        let hash2 = ImageHash::new(vec![vec![false, true], vec![true, false]]);

        // Act
        let distance = hash1.distance(&hash2);

        // Assert
        match distance {
            Ok(d) => assert_eq!(d, 0),
            Err(_) => panic!("Should not have errored"),
        }
    }

    #[test]
    fn test_image_hash_distance_with_different_sizes() {
        // Arrange
        let hash1 = ImageHash::new(vec![vec![false, true, false], vec![true, false, false]]);

        let hash2 = ImageHash::new(vec![vec![false, true], vec![true, false]]);

        // Act
        let distance = hash1.distance(&hash2);

        // Assert
        match distance {
            Ok(_) => panic!("Should not have succeeded"),
            Err(e) => assert_eq!(e, "Cannot compute distance of hashes with different sizes"),
        }
    }
}
