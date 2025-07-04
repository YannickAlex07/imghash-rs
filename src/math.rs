#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub enum Axis {
    #[default]
    Row,
    Column,
}

/// Computes the DCT 2 for a given slice of floats in-place.
///
/// The implementation follows the SciPy implementation.
/// https://docs.scipy.org/doc/scipy/reference/generated/scipy.fftpack.dct.html
///
/// # Arguments
/// * `input`: A mutable reference to a slice of floats.
/// * `skip`: The number of elements to skip between each DCT value.
///           This is used to iterate the elements column-wise.
pub fn dct2_in_place(input: &mut [f64], skip: usize) {
    // we cannot compute the DCT for an empty input
    if input.is_empty() {
        return;
    }

    let n = (input.len() + skip - 1) / skip;

    let dct = (0..n)
        .map(|k| {
            2 as f64
                * input
                    .chunks(skip)
                    .enumerate()
                    .map(|(i, x)| {
                        let numerator = std::f64::consts::PI * k as f64 * (2 * i + 1) as f64;
                        let denominator = (2 * n) as f64;

                        let cosine = (numerator / denominator).cos();

                        x[0] * cosine
                    })
                    .sum::<f64>()
        })
        .collect::<Vec<_>>();

    dct.into_iter().enumerate().for_each(|(i, value)| {
        input[i * skip] = value;
    });
}

/// Computes the DCT 2 in-place over a matrix.
/// The axis controls if the DCT is computed over the columns or over each column.
///
/// # Arguments
/// * `input`: A reference to a matrix of floats
/// * `width`: The width of the matrix
/// * `axis`: The axis over which to compute the DCT 2
pub fn dct2_over_matrix_in_place(input: &mut [f64], width: usize, axis: Axis) {
    // we cannot compute the DCT for an empty matrix
    if input.is_empty() || width == 0 {
        return;
    }

    match axis {
        Axis::Row => input
            .chunks_mut(width)
            .for_each(|row| dct2_in_place(row, 1)),
        Axis::Column => {
            // Step each column of the matrix, skipping `width` elements
            for n in 0..(input.len() / width) {
                dct2_in_place(&mut input[n..], width);
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

    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

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
        dct2_in_place(&mut input, 1);

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
        dct2_in_place(&mut input, 1);

        // Assert
        assert_eq!(input, vec![]);
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
