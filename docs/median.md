# Median Hash

- [Median Hash](#median-hash)
  - [Pros \& Cons](#pros--cons)
      - [Pros](#pros)
      - [Cons](#cons)
  - [Algorithm](#algorithm)
      - [1. Grayscaling \& Resizing](#1-grayscaling--resizing)
      - [2. Calculating Median Brightness](#2-calculating-median-brightness)
      - [3. Compute the Brightness Matrix](#3-compute-the-brightness-matrix)
      - [4. Encoding to Hexadecimal](#4-encoding-to-hexadecimal)

The Median Hash calculates if a given pixel is above or below the median brightness of the image. The result of this is then encoded into a hexadecimal string.

## Pros & Cons

Like each other hashing algorithm, the median hash algorithm has some pros and cons that are important to know and understand.

#### Pros

* Simple and quick to calculate

#### Cons

* Not resiliant to brightness changes


## Algorithm

The algorithm behind median hash is quite simple and consists of the following steps:

1. Grayscale and resize the input image
2. Take the median brightness of all pixels
3. For each pixel compute if the brightness is above or below the median
4. Encode the results into a hexadecimal string

Lets look into each step into more detail and how this crate implements them.

#### 1. Grayscaling & Resizing

The first step is to grayscale and resize the image to a given size. By default this size is 8 x 8 pixels. Final size of the rescaled image is configurable, therefore it is possible to experiment with larger or smaller values to find  the most suitable size for the images you work with.

At best case you want to rescale to a size that is efficient to compute while maintaining enough nuances of the image to make the calculation accurate.

#### 2. Calculating Median Brightness

The next step is to calculate tha median brightness for all pixels. This is a very simple median calculation for which we sort all of the pixels by brightsness and then pick the middle one.

#### 3. Compute the Brightness Matrix

Now that we know the median brightness for the image, we can calculate the brightness matrix. This matrix essentially records which pixel is above or below the given median brightness.

The matrix is simply a 2-dimensional vector of booleans that has the same size as the downscaled image. If a pixel at a certain x and y position is above the median brightness, we will set the given boolean at the same x and y position in the matrix to true.

Let's look at the following example matrix:

$$
\begin{bmatrix}
124 & 096 & 098\\
076 & 089 & 189\\
098 & 073 & 076\\
\end{bmatrix}
$$

If we assume an median brightness of $096$, we get the following brightness matrix:

$$
\begin{bmatrix}
true & false & true\\
false & false & true\\
true & false & false\\
\end{bmatrix}
$$

This resulting matrix can then now be encoded into a hexadecimal result.

#### 4. Encoding to Hexadecimal

Each hasher in the crate returns an `ImageHash`-struct that holds the computed brightness matrix. The `encode`-method can then be used to encode the matrix into a hexadecimal string. You can also use the `decode`-function to decode a string back into its original brightness matrix.

The exact algorithm used to encoding the matrix is described [here](./encoding.md).