# Hash Encoding & Decoding

- [Hash Encoding \& Decoding](#hash-encoding--decoding)
  - [Encoding](#encoding)
  - [Decoding](#decoding)

All of the Hashers in this crate returns an `ImageHash` struct that can be encoded into a hexadecimal string and vice versa.
This document dives a bit deeper into this encoding and explains how it works.

_This encoding and decoding algorithm is explicitly choosen to be compatible with the Python `imagehash` package while also supporting encoding of a non-square matrix. This was done for somewhat personal reason as this crate was set to replace the `imagehash`-package in some places. However the generated hashes by this crate are **not** strictly compatible with the `imagehash`-package._

## Encoding

The encoding algorithm follows the following steps:

1. We flatten the bit matrix into a one dimensional vector of bits
2. Pad the entire vector with leading zeros until it is divisible by 4
3. Go through 4 bits at a time and encode the decimal representation into a hexadecimal character
4. Concatenate all hexadecimal characters into a single string

This encoding is pretty safe and should not fail under normal circumstances - the only exception is when trying to encode an empty vector, for which this crate will throw a panic.

## Decoding

The decoding is slightly more complicated as we have to reshape the vector of bits to a two dimensional vector. We also have to account for the leading zeros that we need to potentially remove again before we actually decode the characters.

The following 3 inputs are required by the decoding algorithm:

* $s$ which is the encoded hexadecimal string
* $w$ which is the width of the decoded matrix
* $h$ which is the height of the decoded matrix

The algorithm then does the following steps:

1. Validate that $s$ is a of valid length that can be encoded into a matrix with the size $w\times{h}$.
2. Compute how many bits were added for leading padding that we need to skip.
3. Decoded each character in the string into its corresponding decimal number
4. Convert the decimal number into a binary representation
5. Concatenate the different binary representations into a long vector of bits
6. Reshape the vector of bits into a two dimensional matrix of bits
7. Make a sanity check that the decoded matrix corresponds to the specified width and height

That is the full algorithm that is used by `imghash` - it includes a set of checks that might return an error in case something went wrong during the encoding.