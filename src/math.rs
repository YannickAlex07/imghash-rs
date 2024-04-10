use std::vec;

#[derive(Debug, PartialEq)]
pub enum Axis {
    Row,
    Column,
}

/// Computes the DCT 2 for a given slice of floats.
/// The implementation follows the SciPy implementation.
/// https://docs.scipy.org/doc/scipy/reference/generated/scipy.fftpack.dct.html
///
/// # Arguments
/// * `input`: A reference to a slice of floats
///
/// # Returns
/// * A vector with the transformed values
pub fn dct2(input: &[f64]) -> Vec<f64> {
    // we cannot compute the DCT for an empty input
    if input.is_empty() {
        return vec![];
    }

    let n = input.len();

    (0..n)
        .map(|k| {
            2 as f64
                * input
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        let numerator = std::f64::consts::PI * k as f64 * (2 * i + 1) as f64;
                        let denominator = (2 * n) as f64;

                        let cosine = (numerator / denominator).cos();

                        x * cosine
                    })
                    .sum::<f64>()
        })
        .collect()
}

/// Computes the DCT 2 over a matrix. The axis controls if the DCT
/// is computed over the columns or over each column.
///
/// # Arguments
/// * `input`: A reference to a matrix of floats
/// * `axis`: The axis over which to compute the DCT 2
///
/// # Returns
/// * A matrix with the modified values
pub fn dct2_over_matrix(input: &Vec<Vec<f64>>, axis: Axis) -> Vec<Vec<f64>> {
    // we cannot compute the DCT for an empty matrix
    if input.is_empty() || input[0].is_empty() {
        return input.clone();
    }

    let mut matrix = input.clone();

    // transpose the matrix if we are computing the DCT over the columns
    if axis == Axis::Column {
        matrix = transpose(&input);
    }

    // iterate and compute dct
    let mut dct_matrix: Vec<Vec<f64>> = vec![];
    for row in matrix {
        let input = row.iter().map(|x| *x as f64).collect::<Vec<f64>>();
        let dct = dct2(&input);

        dct_matrix.push(dct);
    }

    if axis == Axis::Column {
        return transpose(&dct_matrix);
    }

    dct_matrix
}

/// Computes the median for slice of float values.
///
/// # Arguments
/// * `input`: A reference to a slice of floats
///
/// # Returns
/// * Returns a float that represents the median
/// * Returns `None` if `input` is empty
pub fn median(input: &[f64]) -> Option<f64> {
    if input.is_empty() {
        return None;
    }

    let mut sorted = input.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Some((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Some(sorted[mid])
    }
}

/// Transposes a matrix represented as a vector of vectors.
///
/// # Arguments
/// * `input`: A reference to a matrix of floats
///
/// # Returns
/// * A matrix with the transposed values
pub fn transpose(input: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let rows = input.len();
    let cols = input[0].len();

    let mut transposed = vec![vec![0.; rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            transposed[c][r] = input[r][c].clone();
        }
    }

    transposed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dct2() {
        // Arrange
        let input = vec![1., 2., 3., 4.];

        // Act
        let result = dct2(&input);

        // Assert
        assert_eq!(
            result,
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
        let input = vec![];

        // Act
        let result = dct2(&input);

        // Assert
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_dct2_over_matrix_rows() {
        // Arrange
        let input = vec![
            vec![1., 2., 3., 4.],
            vec![5., 6., 7., 8.],
            vec![9., 10., 11., 12.],
            vec![13., 14., 15., 16.],
        ];

        // Act
        let result = dct2_over_matrix(&input, Axis::Row);

        // Assert
        assert_eq!(
            result,
            vec![
                vec![
                    20.0,
                    -6.308644059797899,
                    -1.7763568394002505e-15,
                    -0.44834152916796777
                ],
                vec![
                    52.0,
                    -6.308644059797897,
                    -3.552713678800501e-15,
                    -0.44834152916797
                ],
                vec![
                    84.0,
                    -6.308644059797897,
                    -7.105427357601002e-15,
                    -0.44834152916797265
                ],
                vec![
                    116.0,
                    -6.308644059797899,
                    -3.552713678800501e-15,
                    -0.4483415291679762
                ],
            ]
        );
    }

    #[test]
    fn test_dct2_over_matrix_column() {
        // Arrange
        let input = vec![
            vec![1., 2., 3., 4.],
            vec![5., 6., 7., 8.],
            vec![9., 10., 11., 12.],
            vec![13., 14., 15., 16.],
        ];

        // Act
        let result = dct2_over_matrix(&input, Axis::Column);

        // Assert
        assert_eq!(
            result,
            vec![
                vec![56., 64., 72., 80.,],
                vec![
                    -25.234576239191597,
                    -25.234576239191597,
                    -25.234576239191597,
                    -25.234576239191597
                ],
                vec![
                    -7.105427357601002e-15,
                    -7.105427357601002e-15,
                    -7.105427357601002e-15,
                    -7.105427357601002e-15
                ],
                vec![
                    -1.7933661166718693,
                    -1.7933661166718693,
                    -1.7933661166718693,
                    -1.793366116671871
                ]
            ]
        );
    }

    #[test]
    fn test_dct2_over_matrix_with_empty_rows() {
        // Arrange
        let input = vec![vec![]];

        // Act
        let result = dct2_over_matrix(&input, Axis::Row);

        // Assert
        assert_eq!(result, input);
    }

    #[test]
    fn test_dct2_over_matrix_with_empty_columns() {
        // Arrange
        let input = vec![];

        // Act
        let result = dct2_over_matrix(&input, Axis::Column);

        // Assert
        assert_eq!(result, input);
    }

    #[test]
    fn test_median_with_even_numbers() {
        // Arrange
        let input = vec![3., 2., 1., 4.];

        // Act
        let result = median(&input);

        // Assert
        assert_eq!(result, Some(2.5));
    }

    #[test]
    fn test_median_with_uneven_numbers() {
        // Arrange
        let input = vec![3., 4., 1., 2., 5.];

        // Act
        let result = median(&input);

        // Assert
        assert_eq!(result, Some(3.));
    }

    #[test]
    fn test_median_with_empty_vector() {
        // Arrange
        let input = vec![];

        // Act
        let result = median(&input);

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn test_transpose() {
        // Arrange
        let input = vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]];

        // Act
        let result = transpose(&input);

        // Assert
        assert_eq!(
            result,
            vec![vec![1., 4., 7.], vec![2., 5., 8.], vec![3., 6., 9.]]
        );
    }
}
