# `imghash` - Image Hashing for Rust

[![Crates.io Version](https://img.shields.io/crates/v/imghash)](https://crates.io/crates/imghash)
[![docs.rs](https://img.shields.io/docsrs/imghash)](https://docs.rs/imghash/latest/imghash/)
[![Main](https://github.com/YannickAlex07/imghash-rs/actions/workflows/main.yaml/badge.svg)](https://github.com/YannickAlex07/imghash-rs/actions/workflows/main.yaml)
[![codecov](https://codecov.io/gh/YannickAlex07/imghash-rs/graph/badge.svg?token=df44MdxWix)](https://codecov.io/gh/YannickAlex07/imghash-rs)

- [`imghash` - Image Hashing for Rust](#imghash---image-hashing-for-rust)
  - [Usage](#usage)
    - [Quickstart](#quickstart)
    - [Encoding \& Decoding](#encoding--decoding)
    - [Hamming Distance](#hamming-distance)
    - [Custom Hashers](#custom-hashers)
  - [Python Compatability](#python-compatability)

`imghash` is a crate that allows you to generate different hashes for images. The following hashes can be generated using this crate:

* [Average Hash](./docs/average.md)
* [Difference Hash](./docs/difference.md)
* [Perceptual Hash](./docs/perceptual.md)

## Usage

There are multiple ways how to utilize `imghash` depending on your use case.

### Quickstart

The easy way to use `imghash` is by using the provided utility functions which assume reasonable defaults.

```rust
use imghash::{average_hash, difference_hash, perceptual_hash};

let path = Path::new("path/to/my/image");

let average = average_hash(path);
let difference = difference_hash(path);
let perceptual = perceptual_hash(path);
```

Each of these functions return a `Result<ImageHash, String>`-type. The `ImageHash` object is essentially a container for the encoded bit matrix of the image (learn more [here](./docs/encoding.md)). The `ImageHash` can be encoded into hexadecimal string by calling the `encode`-method:

```rust
let res: String = hash.encode();
```

### Encoding & Decoding

Hashes can be encoded into hexadecimal string by using the `encode()`-method:

```rust
let res: String = hash.encode();
```

A hexadecimal string can also be decoded back into an `ImageHash`:

```rust
let res: Result<ImageHash, String> = hash.decode("24f0", 4, 4);
```

The first argument of the hash is the string, the second and third are the width and height of the underlying matrix. This is required as each string can be encoded into different sizes matricies. If you want to understand more about the underlying bit matrix read the documentation about [encoding](./docs/encoding.md).

### Hamming Distance

The hamming distance is the distance of two hashes defined by the number of bits that differ between them. This distance can be easily computed:

```rust
let distance: Result<usize, String> = hash.distance(other_hash);
```

This can produce an error if the hashes are not of the same size.

### Custom Hashers

If you need more flexibility, for example computing a larger bit matrix than the default, you can use a custom `Hasher`.

For each hash type the crate provides a custom hasher, for the example here we will use the `AverageHasher`:

```rust
use imghash::{average::AverageHasher};

let path = Path::new("path/to/my/image");

let hasher = AverageHasher {
  width: 10,
  height: 10,
};

let hash = hasher.hash_from_path(path);
```

`Hasher`-instances also allow you to create hashes for already loaded images:

```rust
let img = ImageReader::open(...);

let hasher = AverageHasher { ..Default::default() };

let hash = hasher.hash_from_img(&img);
```

Each hasher also implements the `Default`-trait, allowing you to create them with their default values:

```rust
let hasher = AverageHasher { ..Default::default() };
```

## Python Compatability

One of the major factors that drove development of this crate was the need to have a hasher implementation that matches the [`imagehash`-package](https://pypi.org/project/ImageHash/) for Python.

A wrapper with Python-bindings is now available [here](https://github.com/yannickalex07/imghash-py).

As of Version 1.2.0 all hashes generated by this crate should match hashes generated by `imagehash` - however it is not guaranteed for any other package or crate. Previous version of this crate (<1.2.0) were **not** generated the same hashes. To make sure you are generating the same hashes, set the color space of the hasher to `REC601`, as this will make sure the same grayscaling as Pillow is used - this is configured as the default.