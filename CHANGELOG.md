# Version 1.6.0 (WIP)

Contributors: @yannickalex07, @schungx

- Updated perceptual hashing documentation
- Added new `MedianHasher`

# Version 1.5.0

Contributors: @yannickalex07, @schungx

- Updated deprecated `image::io::ImageReader` imports
- Deprecated `bool`-based `ImageHash::new`
- The `ImageHash` type is now backed by a bit vector instead of a `bool` matrix

# Version 1.4.0

Contributors: 

- Updated the internal image crate to version 0.25.6

# Version 1.3.1

Contributors: @yannickalex07

- Added a new `matrix()`-method that allows access to a copy of the underlying bit matrix

# Version 1.3.0

Contributors: @yannickalex07

- Added a new `distance`-method to compute the hamming distance between hashes
- Added a new `shape`-method to get the shape of an underlying bit matrix for a hash
- Made the bit matrix private and added a new `new`-function to create hashes

# Version 1.2.0

Contributors: @yannickalex07

- Introduction of custom grayscaling algorithms. The crate now supports grayscaling using the REC709 or REC601 (new default) color space. This will actually now align the hashes with Python packages like `imagehash` that use Pillow under the hood. However, this will also cause some hashes to be different from previous versions.

# Version 1.1.1

Contributors: @yannickalex07

- Fixed a bug where we didn't transpose a matrix back to its original orientation after applying the dct over the columns. This caused some images to get an invalid hash. This fix however will change all perceptual hashes generated by the perceptual hash function / hasher.

# Version 1.1.0

Contributors: @yannickalex07

- Added workflow for pushing to main

# Version 1.0.0

Contributors: @yannickalex07

- Added hashers for Average, Difference and Perceptual hashes
- Added utility functions for the hashers
- Added ImageHash object for encoding and decoding bit matrices
