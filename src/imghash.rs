#[derive(Debug, PartialEq)]
pub struct ImageHash {
    pub matrix: Vec<Vec<bool>>,
}

impl ImageHash {
    /// Flattens the bit matrix that represents the [`ImageHash`] into a single vector.
    pub fn flatten(&self) -> Vec<bool> {
        self.matrix.iter().flatten().copied().collect()
    }

    /// Encodes the bit matrix that represents the [`ImageHash`] into a hexadecimal string.
    /// This implementation is strictly compatible with `imagehash` package for Python.
    pub fn encode(&self) -> String {
        let mut result = "".to_string();

        let mut flattened = self.flatten();
        if flattened.is_empty() {
            panic!("Cannot encode an empty matrix")
        }

        // the Python package essentially pads the entire bit array with 0s
        // until it is cleanly encodable into hexadecimal characters.
        // this part essentially does the same thing.
        if flattened.len() % 4 != 0 {
            let padding = 4 - (flattened.len() % 4);

            for _ in 0..padding {
                flattened.push(false);
            }

            flattened.rotate_right(padding)
        }

        // we convert the bit array one character at a time
        for chunk in flattened.chunks(4) {
            let byte = chunk.iter().fold(0, |acc, &bit| (acc << 1) | bit as u8);
            result += &format!("{:x}", byte);
        }

        result
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
    /// it allows the decoding of hashes that have been generated on non-square matricies. This is because
    /// the original package actually only allows the generation of hashes on square matricies, however this
    /// crate does allow arbitrary dimensions.
    pub fn decode(s: &str, width: usize, height: usize) -> Result<ImageHash, String> {
        // first we validate that the width and height actually make sense with the given string
        let total_length = width * height;

        // guard against too small values
        if total_length == 0 {
            return Err("Width or height cannot be 0".to_string());
        }

        // validate that s is a valid string
        if s.len() == 0 {
            return Err("String is empty".to_string());
        }

        // guard against a string that is too short or too long for the specified size
        match total_length % 4 {
            0 => {
                if total_length / 4 != s.len() {
                    return Err(
                        "String is too short or too long for the specified size".to_string()
                    );
                }
            }
            remainder => {
                if (total_length + (4 - remainder)) / 4 != s.len() {
                    return Err(
                        "String is too short or too long for the specified size".to_string()
                    );
                }
            }
        }

        // the python package essentially pads the entire bit array with 0s to make
        // it encodable. Here we calculate how many bits were padded, which we can then skip
        // in the beginning.
        let mut skip = 0;
        if total_length % 4 != 0 {
            skip = 4 - ((width * height) % 4);
        }

        // we create a matrix of the correct size
        let mut bits: Vec<bool> = vec![];
        for (i, b) in s.chars().enumerate() {
            let digit = b.to_ascii_lowercase().to_digit(16);
            if digit.is_none() {
                return Err("invalid digit found in string".to_string());
            }

            // we add the necessary skip that we calculated earlier
            // for the first character
            let mut start = 0;
            if i == 0 {
                start += skip;
            }

            // goes through each of the 4 bits that makes up our hexadecimal character
            for i in start..4 {
                // we extract the bit from the digit
                let bit = (digit.unwrap() >> (3 - i)) & 1;
                bits.push(bit == 1)
            }
        }

        let matrix: Vec<Vec<bool>> = bits.chunks(width).map(|x: &[bool]| x.to_vec()).collect();

        // sanity checks
        if matrix.len() != height || matrix.last().unwrap().len() != width {
            return Err(
                "Matrix dimensions do not match the specified width and height".to_string(),
            );
        }

        Ok(ImageHash { matrix })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // FLATTEN

    #[test]
    fn test_image_hash_flatten() {
        // Arrange
        let hash = ImageHash {
            matrix: vec![vec![false, true], vec![true, false]],
        };

        let expected = vec![false, true, true, false];

        // Act
        let flattened = hash.flatten();

        // Assert
        assert_eq!(flattened, expected);
    }

    // PYTHON SAFE ENCODING

    #[test]
    fn test_image_hash_encoding() {
        // Arrange

        // -> resulting bit str: 0010 0100 1111 0000
        // -> resulting hex str: 24F0
        let hash = ImageHash {
            matrix: vec![
                vec![false, false, true, false],
                vec![false, true, false, false],
                vec![true, true, true, true],
                vec![false, false, false, false],
            ],
        };

        // Assert
        assert_eq!(hash.encode(), "24f0");
    }

    #[test]
    fn test_image_hash_encoding_with_non_square_matrix() {
        // Arrange

        // -> resulting bit str: 0110 1010 0011 1110 0001
        // -> resulting hex str: 6A3E1
        let hash = ImageHash {
            matrix: vec![
                vec![false, true, true, false, true],
                vec![false, true, false, false, false],
                vec![true, true, true, true, true],
                vec![false, false, false, false, true],
            ],
        };

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
        let hash = ImageHash {
            matrix: vec![
                vec![false, true, true, false, true],
                vec![false, true, false, false, false],
                vec![true, true, true, true, true],
            ],
        };

        // Assert
        assert_eq!(hash.encode(), "351f");
    }

    #[test]
    #[should_panic(expected = "Cannot encode an empty matrix")]
    fn test_image_hash_encoding_with_empty_matrix() {
        // Arrange
        let hash = ImageHash { matrix: vec![] };

        // Assert
        hash.encode(); // <- should panic
    }

    #[test]
    fn test_image_hash_python_safe_encoding_with_single_bit() {
        // Arrange

        // should equal to 1 due to added padding
        // -> resulting bit str: 0001
        // -> resulting hex str: 1
        let hash = ImageHash {
            matrix: vec![vec![true]],
        };

        // Assert
        assert_eq!(hash.encode(), "1");
    }

    // PYTHON SAFE DECODING

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
        assert_eq!(decoded.matrix, expected);
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
        assert_eq!(decoded.matrix, expected);
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
        assert_eq!(decoded.matrix, expected);
    }

    #[test]
    fn test_image_hash_decoding_with_single_bit() {
        // Arrange
        let expected = vec![vec![true]];

        // Act
        let decoded = ImageHash::decode("1", 1, 1).unwrap();

        // Assert
        assert_eq!(decoded.matrix, expected);
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
}
