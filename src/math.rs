#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub enum Axis {
    #[default]
    Row,
    Column,
}

/// Computes the DCT Type-II for a given slice of floats in-place.
///
/// The Discrete Cosine Transform (DCT) converts spatial data (like pixel values)
/// into frequency components. Low-frequency components capture the overall structure
/// of the signal, while high-frequency components capture fine detail. This property
/// is what makes DCT useful for perceptual hashing: by keeping only the low-frequency
/// components, we get a compact representation of the image's structure that is robust
/// to small changes.
///
/// The formula implemented is (following SciPy's convention):
///
///   Y[k] = 2 * sum_{i=0}^{N-1} x[i] * cos(pi * k * (2i + 1) / (2N))
///
/// where N is the number of elements and k is the output frequency index.
///
/// See: https://docs.scipy.org/doc/scipy/reference/generated/scipy.fftpack.dct.html
///
/// # Arguments
/// * `input`: A mutable reference to a slice of floats. Results are written back here.
/// * `skip`: Stride between elements. Use `1` for contiguous (row-wise) data, or
///           `width` to step through a single column of a row-major matrix.
/// * `buf`: Temporary buffer for intermediate results. Must be at least N elements long.
pub fn dct2_in_place(input: &mut [f64], skip: usize, buf: &mut [f64]) {
    // Internal invariant: all callers control `skip` directly (1 for rows, `width` for columns).
    // A zero skip is a programming bug, not a recoverable error.
    assert!(skip > 0, "skip value must be greater than 0");

    if input.is_empty() {
        return;
    }

    // Number of logical elements to transform.
    // When skip > 1 (column mode), elements are spaced `skip` apart in the flat array,
    // so we divide the total length by the stride to get the element count.
    let n = (input.len() + skip - 1) / skip;

    // Internal invariant: callers are responsible for allocating a buffer that fits the result.
    // A too-small buffer is a programming bug, not a recoverable error.
    assert!(n <= buf.len(), "buffer is too small for the DCT result");

    // For each output frequency index k, compute the DCT coefficient.
    // Each coefficient is a weighted sum of all input values, where the weights
    // are cosine basis functions at increasing frequencies.
    (0..n)
        .map(|k| {
            2.0 * input
                // chunks(skip) gives us windows of `skip` elements; we only use
                // the first element of each chunk (x[0]), effectively stepping
                // through the array with the given stride.
                .chunks(skip)
                .enumerate()
                .map(|(i, x)| {
                    // cos(pi * k * (2i+1) / 2N) is the DCT-II basis function.
                    // - k selects the frequency (0 = DC / average, higher = finer detail)
                    // - i is the position of the current input sample
                    let numerator = std::f64::consts::PI * k as f64 * (2 * i + 1) as f64;
                    let denominator = (2 * n) as f64;

                    let cosine = (numerator / denominator).cos();

                    x[0] * cosine
                })
                .sum::<f64>()
        })
        .enumerate()
        .for_each(|(i, value)| buf[i] = value);

    // Copy the results from the temporary buffer back into `input`,
    // respecting the original stride so that column-mode writes go
    // to the correct positions in the matrix.
    input
        .chunks_mut(skip)
        .zip(buf.iter().copied())
        .for_each(|(x, value)| {
            x[0] = value;
        });
}

/// Computes the DCT Type-II in-place over a 2D matrix stored as a flat array (row-major).
///
/// For perceptual hashing, this is typically applied twice: once along rows, then along
/// columns (or vice versa), to produce a 2D DCT. The top-left corner of the result
/// contains the lowest-frequency components that summarize the image's overall structure.
///
/// # Arguments
/// * `input`: A flat row-major matrix of floats (length = rows * width).
/// * `width`: The number of columns in the matrix.
/// * `axis`: Which direction to apply the DCT:
///   - `Axis::Row`: transform each row independently (left-to-right frequencies).
///   - `Axis::Column`: transform each column independently (top-to-bottom frequencies).
pub fn dct2_over_matrix_in_place(input: &mut [f64], width: usize, axis: Axis) {
    if input.is_empty() || width == 0 {
        return;
    }

    match axis {
        Axis::Row => {
            // Process each row as a contiguous slice of `width` elements.
            // skip=1 because elements within a row are adjacent in memory.
            let buf = &mut vec![0.0; width];
            for row in input.chunks_mut(width) {
                dct2_in_place(row, 1, buf);
            }
        }
        Axis::Column => {
            // To process a column in a row-major layout, we start at the column's
            // index (n) and skip `width` elements to reach the next row's value in
            // the same column. The `skip` parameter of dct2_in_place handles this stride.
            let buf = &mut vec![0.0; input.len() / width];
            for n in 0..width {
                dct2_in_place(&mut input[n..], width, buf);
            }
        }
    }
}

/// Computes the median for slice of float values.
///
/// # Arguments
/// * `input`: A reference to a slice of floats
///
/// # Returns
/// * Returns a float that represents the median
/// * Returns `None` if `input` is empty
pub fn median(input: impl IntoIterator<Item = f64>) -> Option<f64> {
    let mut sorted = input.into_iter().collect::<Vec<_>>();

    if sorted.is_empty() {
        return None;
    }

    sorted.sort_by(|a, b| a.total_cmp(b));

    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Some((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Some(sorted[mid])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dct2() {
        // Arrange
        let mut input = vec![1., 2., 3., 4.];

        // Act
        let buf = &mut vec![0.0; input.len()];
        dct2_in_place(&mut input, 1, buf);

        // Assert
        assert_eq!(
            input,
            vec![
                20.0,
                -6.308644059797899,
                -1.7763568394002505e-15,
                -0.44834152916796777
            ]
        );
    }

    #[test]
    fn test_dct2_with_empty_input() {
        // Arrange
        let mut input = vec![];

        // Act
        let buf = &mut vec![0.0; input.len()];
        dct2_in_place(&mut input, 1, buf);

        // Assert
        assert_eq!(input, vec![]);
    }

    #[test]
    #[should_panic(expected = "skip value must be greater than 0")]
    fn test_dct2_with_zero_skip() {
        let mut input = vec![1., 2., 3.];
        let buf = &mut vec![0.0; input.len()];
        dct2_in_place(&mut input, 0, buf);
    }

    #[test]
    #[should_panic(expected = "buffer is too small")]
    fn test_dct2_with_small_buffer() {
        let mut input = vec![1., 2., 3., 4.];
        let buf = &mut vec![0.0; 1];
        dct2_in_place(&mut input, 1, buf);
    }

    #[test]
    fn test_dct2_over_matrix_rows() {
        // Arrange
        let mut input = vec![
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
        ];

        // Act
        dct2_over_matrix_in_place(&mut input, 4, Axis::Row);

        // Assert
        assert_eq!(
            input,
            vec![
                20.0,
                -6.308644059797899,
                -1.7763568394002505e-15,
                -0.44834152916796777,
                52.0,
                -6.308644059797897,
                -3.552713678800501e-15,
                -0.44834152916797,
                84.0,
                -6.308644059797897,
                -7.105427357601002e-15,
                -0.44834152916797265,
                116.0,
                -6.308644059797899,
                -3.552713678800501e-15,
                -0.4483415291679762
            ]
        );
    }

    #[test]
    fn test_dct2_over_matrix_column() {
        // Arrange
        let mut input = vec![
            1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.,
        ];

        // Act
        dct2_over_matrix_in_place(&mut input, 4, Axis::Column);

        // Assert
        assert_eq!(
            input,
            vec![
                56.,
                64.,
                72.,
                80.,
                -25.234576239191597,
                -25.234576239191597,
                -25.234576239191597,
                -25.234576239191597,
                -7.105427357601002e-15,
                -7.105427357601002e-15,
                -7.105427357601002e-15,
                -7.105427357601002e-15,
                -1.7933661166718693,
                -1.7933661166718693,
                -1.7933661166718693,
                -1.793366116671871
            ]
        );
    }

    #[test]
    fn test_dct2_over_matrix_with_empty_rows() {
        // Arrange
        let mut input = vec![];

        // Act
        dct2_over_matrix_in_place(&mut input, 0, Axis::Row);

        // Assert
        assert_eq!(input, vec![]);
    }

    #[test]
    fn test_dct2_over_matrix_with_empty_columns() {
        // Arrange
        let mut input = vec![];

        // Act
        dct2_over_matrix_in_place(&mut input, 0, Axis::Column);

        // Assert
        assert_eq!(input, vec![]);
    }

    #[test]
    fn test_median_with_even_numbers() {
        // Arrange
        let input = vec![3., 2., 1., 4.];

        // Act
        let result = median(input);

        // Assert
        assert_eq!(result, Some(2.5));
    }

    #[test]
    fn test_median_with_uneven_numbers() {
        // Arrange
        let input = vec![3., 4., 1., 2., 5.];

        // Act
        let result = median(input);

        // Assert
        assert_eq!(result, Some(3.));
    }

    #[test]
    fn test_median_with_empty_vector() {
        // Arrange
        let input = vec![];

        // Act
        let result = median(input);

        // Assert
        assert_eq!(result, None);
    }
}
