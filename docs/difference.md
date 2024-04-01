# Difference Hash

- [Difference Hash](#difference-hash)
  - [Pros \& Cons](#pros--cons)
      - [Pros](#pros)
      - [Cons](#cons)
  - [Algorithm](#algorithm)
      - [1. Grayscaling \& Resizing](#1-grayscaling--resizing)
      - [2. Calculating Differences between Neighbours](#2-calculating-differences-between-neighbours)
      - [3. Encoding to Hexadecimal](#3-encoding-to-hexadecimal)

The Difference Hash calculates the brightness difference between neighbouring pixels and encodes them inta a hash.

## Pros & Cons

Like each other hashing algorithm, the difference hash algorithm has some pros and cons that are important to know and understand.

#### Pros

* Simple and quick to calculate

#### Cons

* ...


## Algorithm

The algorithm behind average hash is quite simple and consists of the following steps:

1. Grayscale and resize the input image
2. Calculate Differences between neighbouring pixels
3. Encode the results into a hexadecimal string

Lets look into each step into more detail and how this crate implements them.

#### 1. Grayscaling & Resizing

The first step is to grayscale and resize the image to a given size. By default this size is 9 x 8 pixels. Because the difference hash looks and neighbouring pixels we need to scale it by 1 pixel more on one edge, so that the resulting matrix is still square. This means the default scaling of size 9 x 8 will give us a resulting matrix of 8 x 8.

As with every algorithm, the scale is configurable and you can experiment with it to find the best possible size for your use case.

_Keep in mind here that the crate allows you to configure the size of the resulting matrix and **not** of the rescaled image. The crate will rescale the image to `width + 1` and your configured `height` to calculate the difference hash._

#### 2. Calculating Differences between Neighbours

Second step for calculating the difference hash is to compute the difference between neighbouring values by checking if the current pixel is **less bright** than the pixel next to it.

Take the following image matrix as an example:

$$
\begin{bmatrix}
124 & 096 & 098 & 067\\
076 & 089 & 189 & 176\\
098 & 073 & 076 & 023\\
\end{bmatrix}
$$

From this we would get the resulting matrix:

$$
\begin{bmatrix}
false & true & false\\
true & true & false\\
false & true & false\\
\end{bmatrix}
$$

If you take the first row as an example you can see that $124$ is not smaller than $096$, therefore the resulting value will be false. We then continue doing this for each value in each row.

This matrix can then be encoding into a final hexadecimal string.

#### 3. Encoding to Hexadecimal

Each hasher in the crate returns an `ImageHash`-struct that holds the computed brightness matrix. The `encode`-method can then be used to encode the matrix into a hexadecimal string. You can also use the `decode`-function to decode a string back into its original brightness matrix.

The exact algorithm used to encoding the matrix is described [here](./encoding.md).