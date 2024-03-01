use std::vec;

#[derive(Debug, PartialEq)]
pub enum Axis {
    Row,
    Column,
}

pub fn dct2(input: &[f64]) -> Vec<f64> {
    // we cannot compute the DCT for an empty input
    if input.is_empty() {
        return vec![];
    }

    let n = input.len();

    (0..n)
        .map(|k| {
            input
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    let numerator = std::f64::consts::PI * k as f64 * (2 * i + 1) as f64;
                    let denominator = (2 * n) as f64;

                    let cosine = (numerator / denominator).cos();

                    2 as f64 * x * cosine
                })
                .sum()
        })
        .collect()
}

pub fn dct2_over_matrix(input: &Vec<Vec<f64>>, axis: Axis) -> Vec<Vec<f64>> {
    // we cannot compute the DCT for an empty matrix
    if input.is_empty() || input[0].is_empty() {
        return input.clone();
    }

    let mut matrix = input.clone();

    // transpose the matrix if we are computing the DCT over the columns
    if axis == Axis::Column {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut transposed = vec![vec![matrix[0][0].clone(); rows]; cols];
        for r in 0..rows {
            for c in 0..cols {
                transposed[c][r] = matrix[r][c].clone();
            }
        }

        matrix = transposed;
    }

    // iterate and compute dct
    let mut dct_matrix: Vec<Vec<f64>> = vec![];
    for row in matrix {
        let input = row.iter().map(|x| *x as f64).collect::<Vec<f64>>();
        let dct = dct2(&input);

        dct_matrix.push(dct);
    }

    dct_matrix
}

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
    fn test_dct2_over_matrix_rows() {
        // Arrange
        let input = vec![vec![1., 2.], vec![3., 4.]];

        // Act
        let result = dct2_over_matrix(&input, Axis::Row);

        // Assert
        assert_eq!(
            result,
            vec![[6.0, -1.4142135623730947], [14.0, -1.414213562373094]]
        );
    }

    #[test]
    fn test_dct2_over_matrix_column() {
        // Arrange
        let input = vec![vec![1., 2.], vec![3., 4.]];

        // Act
        let result = dct2_over_matrix(&input, Axis::Column);

        // Assert
        assert_eq!(
            result,
            vec![[8.0, -2.82842712474619], [12.0, -2.8284271247461894]]
        );
    }
}
