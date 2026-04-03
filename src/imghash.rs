use std::io;
use std::path::PathBuf;

use bitvec::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ImageHashError {
    #[error("Failed to read image from path '{}': {source}", path.display())]
    IoError { source: io::Error, path: PathBuf },

    #[error("Failed to decode image: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("Matrix cannot be empty")]
    EmptyMatrix,

    #[error("Iterator yielded {actual} elements, expected {expected} (width * height)")]
    IteratorLengthMismatch { expected: usize, actual: usize },

    #[error("Cannot compute distance: hash shapes differ ({self_shape:?} vs {other_shape:?})")]
    ShapeMismatch {
        self_shape: (usize, usize),
        other_shape: (usize, usize),
    },

    #[error("Hex string length {actual} does not match expected {expected} nibbles for {width}x{height} hash")]
    InvalidHashLength {
        expected: usize,
        actual: usize,
        width: u32,
        height: u32,
    },

    #[error("Invalid hexadecimal character in hash string")]
    InvalidHexCharacter,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageHash {
    // The internal bit-vector stored in LSB order.
    data: BitBox<u8, Lsb0>,

    // Number of columns.
    width: u32,
}

impl ImageHash {
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
    ) -> Result<ImageHash, ImageHashError> {
        let length = width as usize * height as usize;
        if length == 0 {
            return Err(ImageHashError::EmptyMatrix);
        }

        let mut data = bitbox![u8, Lsb0; 0; length];
        let mut count = 0;
        let mut iter = iter.into_iter();

        for i in 0..length {
            match iter.next() {
                Some(bit) => {
                    data.set(i, bit);
                    count += 1;
                }
                None => break,
            }
        }

        // Check if the iterator had fewer elements than expected
        if count != length {
            return Err(ImageHashError::IteratorLengthMismatch {
                expected: length,
                actual: count,
            });
        }

        // Check if the iterator has leftover elements (more than expected).
        // We intentionally avoid calling iter.count() here because the iterator
        // may be unbounded, which would cause an infinite loop.
        if iter.next().is_some() {
            return Err(ImageHashError::IteratorLengthMismatch {
                expected: length,
                actual: length + 1,
            });
        }

        Ok(ImageHash { data, width })
    }

    /// Create an iterator yielding `bool` values over the bits of an [`ImageHash`].
    ///
    /// # Returns
    /// * Bits from the [`ImageHash`], by column then by row.
    pub fn iter_bool(&self) -> impl Iterator<Item = bool> + '_ {
        self.data.iter().by_vals()
    }

    /// The shape of the matrix that represents the [`ImageHash`], in (number of rows, number of columns).
    pub fn shape(&self) -> (usize, usize) {
        (self.data.len() / self.width as usize, self.width as usize)
    }

    /// The hamming distance between this hash and the other hash.
    /// The hamming distance is the number of bits that differ between the two hashes.
    pub fn distance(&self, other: &ImageHash) -> Result<usize, ImageHashError> {
        if self.shape() != other.shape() {
            return Err(ImageHashError::ShapeMismatch {
                self_shape: self.shape(),
                other_shape: other.shape(),
            });
        }

        Ok(self
            .data
            .as_raw_slice()
            .iter()
            .zip(other.data.as_raw_slice().iter())
            .map(|(a, b)| (a ^ b).count_ones() as usize)
            .sum())
    }

    /// Encodes the bit matrix that represents the [`ImageHash`] into a hexadecimal string.
    /// This implementation is strictly compatible with `imagehash` package for Python.
    pub fn encode(&self) -> Result<String, ImageHashError> {
        use std::io::Write;

        if self.data.is_empty() || self.width == 0 {
            return Err(ImageHashError::EmptyMatrix);
        }

        let mut result = Vec::new();

        let length = self.data.len();
        let size = (length + 7) / 8;
        let padding = (size * 8) - length;
        let nibbles = (length + 3) / 4;
        let odd = nibbles % 2 == 1;

        let buffer = BitBox::<u8, Msb0>::from_iter(
            std::iter::repeat_n(false, padding).chain(self.iter_bool()),
        );

        for byte in buffer.as_raw_slice().iter() {
            // Skip the leading '0' if the number of nibbles is odd
            if odd && result.is_empty() {
                write!(&mut result, "{:01x}", byte).unwrap();
            } else {
                write!(&mut result, "{:02x}", byte).unwrap();
            }
        }

        // Infallible: hex formatting only produces valid UTF-8 ASCII bytes
        Ok(String::from_utf8(result).unwrap())
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
    pub fn decode(s: &str, width: u32, height: u32) -> Result<ImageHash, ImageHashError> {
        let length = width as usize * height as usize;

        if length == 0 {
            return Err(ImageHashError::EmptyMatrix);
        }

        let size = (length + 7) / 8;
        let nibbles = (length + 3) / 4;
        let padding = (size * 8) - length;

        if s.len() != nibbles {
            return Err(ImageHashError::InvalidHashLength {
                expected: nibbles,
                actual: s.len(),
                width,
                height,
            });
        }

        // Add padding if the number of nibbles is odd
        let mut iter =
            std::iter::repeat_n('0', if nibbles % 2 == 1 { 1 } else { 0 }).chain(s.chars());

        let mut data = Vec::<u8>::with_capacity(size);

        for _ in 0..size {
            let hi = iter
                .next()
                .unwrap()
                .to_digit(16)
                .ok_or(ImageHashError::InvalidHexCharacter)?;

            let lo = iter
                .next()
                .unwrap()
                .to_digit(16)
                .ok_or(ImageHashError::InvalidHexCharacter)?;

            let value = ((hi << 4) + lo) as u8;
            data.push(value);
        }

        let data =
            BitBox::<u8, Lsb0>::from_iter(data.view_bits::<Msb0>()[padding..].iter().by_vals());

        Ok(ImageHash { data, width })
    }
}

impl std::fmt::Display for ImageHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.encode() {
            Ok(s) => write!(f, "{}", s),
            Err(e) => write!(f, "<invalid hash: {}>", e),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // SHAPE

    #[test]
    fn test_image_hash_shape() {
        // Arrange
        let hash = ImageHash::from_bool_iter(vec![false, true, true, false], 2, 2).unwrap();

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
        let hash = ImageHash::from_bool_iter(
            vec![
                false, false, true, false, //
                false, true, false, false, //
                true, true, true, true, //
                false, false, false, false,
            ],
            4,
            4,
        );

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), "24f0");
    }

    #[test]
    fn test_image_hash_encoding_with_non_square_matrix() {
        // Arrange

        // -> resulting bit str: 0110 1010 0011 1110 0001
        // -> resulting hex str: 6A3E1
        let hash = ImageHash::from_bool_iter(
            vec![
                false, true, true, false, true, //
                false, true, false, false, false, //
                true, true, true, true, true, //
                false, false, false, false, true,
            ],
            5,
            4,
        );

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), "6a3e1");
    }

    #[test]
    fn test_image_hash_encoding_with_uneven_total_bits() {
        // Arrange

        // due to the uneven number of bits, the entire bit string gets padded until
        // it is divisible by 4
        // -> resulting bit str: 0011 0101 0001 1111
        // -> resulting hex str: 351F
        let hash = ImageHash::from_bool_iter(
            vec![
                false, true, true, false, true, //
                false, true, false, false, false, //
                true, true, true, true, true, //
            ],
            5,
            3,
        );

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), "351f");
    }

    #[test]
    fn test_image_hash_encoding_with_empty_matrix() {
        let result = ImageHash::from_bool_iter(vec![], 0, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_image_hash_python_safe_encoding_with_single_bit() {
        // Arrange

        // should equal to 1 due to added padding
        // -> resulting bit str: 0001
        // -> resulting hex str: 1
        let hash = ImageHash::from_bool_iter(vec![true], 1, 1);

        // Assert
        assert!(hash.is_ok());
        assert_eq!(hash.unwrap().encode().unwrap(), "1");
    }

    // DECODING

    #[test]
    fn test_image_hash_decoding() {
        // Arrange
        let expected = vec![
            false, false, true, false, //
            false, true, false, false, //
            true, true, true, true, //
            false, false, false, false, //
        ];

        // Act
        let decoded = ImageHash::decode("24f0", 4, 4);

        // Assert
        assert!(decoded.is_ok());
        assert_eq!(
            decoded.unwrap().iter_bool().collect::<Vec<bool>>(),
            expected
        );
    }

    #[test]
    fn test_image_hash_decoding_with_non_square_matrix() {
        // Arrange
        let expected = vec![
            false, true, true, false, true, //
            false, true, false, false, false, //
            true, true, true, true, true, //
            false, false, false, false, true, //
        ];

        // Act
        let decoded = ImageHash::decode("6a3e1", 5, 4);

        // Assert
        assert!(decoded.is_ok());
        assert_eq!(
            decoded.unwrap().iter_bool().collect::<Vec<bool>>(),
            expected
        );
    }

    #[test]
    fn test_image_hash_decoding_with_uneven_total_bits() {
        // Arrange
        let expected = vec![
            false, true, true, false, true, //
            false, true, false, false, false, //
            true, true, true, true, true, //
        ];

        // Act
        let decoded = ImageHash::decode("351f", 5, 3);

        // Assert
        assert!(decoded.is_ok());
        assert_eq!(
            decoded.unwrap().iter_bool().collect::<Vec<bool>>(),
            expected
        );
    }

    #[test]
    fn test_image_hash_decoding_with_single_bit() {
        // Arrange
        let expected = vec![true];

        // Act
        let decoded = ImageHash::decode("1", 1, 1);

        // Assert
        assert!(decoded.is_ok());
        assert_eq!(
            decoded.unwrap().iter_bool().collect::<Vec<bool>>(),
            expected
        );
    }

    #[test]
    fn test_image_hash_decoding_with_too_short_string() {
        // Act
        let decoded = ImageHash::decode("AB", 2, 5);

        // Assert
        assert!(decoded.is_err());
    }

    #[test]
    fn test_image_hash_decoding_with_too_long_string() {
        // Act
        let decoded = ImageHash::decode("ABCD", 2, 2);

        // Assert
        assert!(decoded.is_err());
    }

    #[test]
    fn test_image_hash_decoding_with_invalid_string() {
        // Act
        let decoded = ImageHash::decode("!", 2, 2);

        // Assert
        assert!(decoded.is_err());
    }

    #[test]
    fn test_image_hash_decoding_with_zero_size_matrix() {
        // Act
        let decoded = ImageHash::decode("!", 2, 0);

        // Assert
        assert!(decoded.is_err());
    }

    #[test]
    fn test_image_hash_decoding_with_empty_string() {
        // Act
        let decoded = ImageHash::decode("", 2, 2);

        // Assert
        assert!(decoded.is_err());
    }

    // DISTANCE

    #[test]
    fn test_image_hash_from_bool_iter_with_too_many_elements() {
        // Arrange: 2x2 = 4 expected, but we supply 6
        let result = ImageHash::from_bool_iter(vec![true, false, true, false, true, true], 2, 2);

        // Assert
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            ImageHashError::IteratorLengthMismatch {
                expected: 4,
                actual: 5,
            }
        ));
    }

    #[test]
    fn test_image_hash_from_bool_iter_with_one_extra_element() {
        // Arrange: 1x1 = 1 expected, but we supply 2
        let result = ImageHash::from_bool_iter(vec![true, false], 1, 1);

        // Assert
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            ImageHashError::IteratorLengthMismatch {
                expected: 1,
                actual: 2,
            }
        ));
    }

    #[test]
    fn test_image_hash_from_bool_iter_with_unbounded_iterator() {
        // Arrange: 2x2 = 4 expected, but we supply an infinite iterator
        let result = ImageHash::from_bool_iter(std::iter::repeat(true), 2, 2);

        // Assert: must return an error without hanging
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            ImageHashError::IteratorLengthMismatch {
                expected: 4,
                actual: 5,
            }
        ));
    }

    // DISTANCE

    #[test]
    fn test_image_hash_distance_with_unequal_hashes() {
        // Arrange
        let hash1 = ImageHash::from_bool_iter(
            vec![
                false, true, true, //
                true, false, false, //
                true, false, true,
            ],
            3,
            3,
        )
        .unwrap();

        let hash2 = ImageHash::from_bool_iter(
            vec![
                true, true, true, //
                false, false, false, //
                true, false, true,
            ],
            3,
            3,
        )
        .unwrap();

        // Act
        let distance = hash1.distance(&hash2);

        // Assert
        assert!(distance.is_ok());
        assert_eq!(distance.unwrap(), 2);
    }

    #[test]
    fn test_image_hash_distance_with_equal_hashes() {
        // Arrange
        let hash1 = ImageHash::from_bool_iter(vec![false, true, true, false], 2, 2).unwrap();

        let hash2 = ImageHash::from_bool_iter(vec![false, true, true, false], 2, 2).unwrap();

        // Act
        let distance = hash1.distance(&hash2);

        // Assert
        assert!(distance.is_ok());
        assert_eq!(distance.unwrap(), 0);
    }

    #[test]
    fn test_image_hash_from_bool_iter_with_too_few_elements() {
        // Arrange: 3x2 = 6 expected, but we supply 4
        let result = ImageHash::from_bool_iter(vec![true, false, true, false], 3, 2);

        // Assert
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(
            err,
            ImageHashError::IteratorLengthMismatch {
                expected: 6,
                actual: 4,
            }
        ));
    }

    // ROUNDTRIP

    #[test]
    fn test_image_hash_encode_decode_roundtrip() {
        // Arrange
        let original = ImageHash::from_bool_iter(
            vec![
                false, false, true, false, //
                false, true, false, false, //
                true, true, true, true, //
                false, false, false, false,
            ],
            4,
            4,
        )
        .unwrap();

        // Act
        let encoded = original.encode().unwrap();
        let decoded = ImageHash::decode(&encoded, 4, 4).unwrap();

        // Assert
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_image_hash_encode_decode_roundtrip_non_square() {
        // Arrange
        let original = ImageHash::from_bool_iter(
            vec![
                false, true, true, false, true, //
                false, true, false, false, false, //
                true, true, true, true, true, //
                false, false, false, false, true,
            ],
            5,
            4,
        )
        .unwrap();

        // Act
        let encoded = original.encode().unwrap();
        let decoded = ImageHash::decode(&encoded, 5, 4).unwrap();

        // Assert
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_image_hash_encode_decode_roundtrip_uneven_bits() {
        // Arrange: 5x3 = 15 bits (not divisible by 4 or 8)
        let original = ImageHash::from_bool_iter(
            vec![
                false, true, true, false, true, //
                false, true, false, false, false, //
                true, true, true, true, true,
            ],
            5,
            3,
        )
        .unwrap();

        // Act
        let encoded = original.encode().unwrap();
        let decoded = ImageHash::decode(&encoded, 5, 3).unwrap();

        // Assert
        assert_eq!(original, decoded);
    }

    // DISTANCE (continued)

    #[test]
    fn test_image_hash_distance_is_symmetric() {
        // Arrange
        let hash1 = ImageHash::from_bool_iter(
            vec![
                false, true, true, //
                true, false, false, //
                true, false, true,
            ],
            3,
            3,
        )
        .unwrap();

        let hash2 = ImageHash::from_bool_iter(
            vec![
                true, true, true, //
                false, false, false, //
                true, false, true,
            ],
            3,
            3,
        )
        .unwrap();

        // Act & Assert
        assert_eq!(
            hash1.distance(&hash2).unwrap(),
            hash2.distance(&hash1).unwrap()
        );
    }

    #[test]
    fn test_image_hash_distance_all_bits_differ() {
        // Arrange: all true vs all false => distance = 4
        let hash1 = ImageHash::from_bool_iter(vec![true, true, true, true], 2, 2).unwrap();

        let hash2 = ImageHash::from_bool_iter(vec![false, false, false, false], 2, 2).unwrap();

        // Act
        let distance = hash1.distance(&hash2).unwrap();

        // Assert
        assert_eq!(distance, 4);
    }

    // DISPLAY

    #[test]
    fn test_image_hash_display() {
        // Arrange
        let hash = ImageHash::from_bool_iter(
            vec![
                false, false, true, false, //
                false, true, false, false, //
                true, true, true, true, //
                false, false, false, false,
            ],
            4,
            4,
        )
        .unwrap();

        // Act
        let display = format!("{}", hash);

        // Assert
        assert_eq!(display, "24f0");
    }

    #[test]
    fn test_image_hash_distance_with_different_sizes() {
        // Arrange
        let hash1 =
            ImageHash::from_bool_iter(vec![false, true, false, true, false, false], 3, 2).unwrap();

        let hash2 = ImageHash::from_bool_iter(vec![false, true, true, false], 2, 2).unwrap();

        // Act & Assert
        assert!(hash1.distance(&hash2).is_err());
    }
}
