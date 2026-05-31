#![allow(clippy::needless_range_loop)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DenseMatrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl DenseMatrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = data.first().map(|r| r.len()).unwrap_or(0);
        Self { rows, cols, data }
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.data[i][i] = 1.0;
        }
        m
    }

    pub fn multiply(&self, other: &DenseMatrix) -> DenseMatrix {
        let mut result = DenseMatrix::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        result
    }

    pub fn transpose(&self) -> DenseMatrix {
        let mut result = DenseMatrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[j][i] = self.data[i][j];
            }
        }
        result
    }

    /// Gaussian elimination to find kernel basis.
    pub fn kernel_basis(&self) -> Vec<Vec<f64>> {
        let mut aug = self.data.clone();
        let n = self.cols;
        let m = self.rows;
        let mut pivot_cols: Vec<Option<usize>> = vec![None; m];
        let mut pivot_row = 0;

        for col in 0..n {
            if pivot_row >= m { break; }
            // Find pivot
            let mut found = None;
            for row in pivot_row..m {
                if aug[row][col].abs() > 1e-10 {
                    found = Some(row);
                    break;
                }
            }
            if let Some(row) = found {
                aug.swap(pivot_row, row);
                let scale = aug[pivot_row][col];
                for j in 0..n {
                    aug[pivot_row][j] /= scale;
                }
                for row in 0..m {
                    if row != pivot_row && aug[row][col].abs() > 1e-10 {
                        let factor = aug[row][col];
                        for j in 0..n {
                            aug[row][j] -= factor * aug[pivot_row][j];
                        }
                    }
                }
                pivot_cols[pivot_row] = Some(col);
                pivot_row += 1;
            }
        }

        let pivot_col_set: std::collections::HashSet<usize> = pivot_cols.iter()
            .filter_map(|&c| c)
            .collect();

        let mut basis = Vec::new();
        for col in 0..n {
            if !pivot_col_set.contains(&col) {
                let mut v = vec![0.0; n];
                v[col] = 1.0;
                for (row, pc) in pivot_cols.iter().enumerate() {
                    if let Some(pc) = pc {
                        v[*pc] = -aug[row][col];
                    }
                }
                basis.push(v);
            }
        }
        basis
    }

    /// Image basis via row echelon form of transpose.
    pub fn image_basis(&self) -> Vec<Vec<f64>> {
        let t = self.transpose();
        let kb = t.kernel_basis();
        // If transpose has trivial kernel, original is full rank
        let kernel_dim = kb.len();
        let rank = self.cols.saturating_sub(kernel_dim);
        // Extract linearly independent columns
        let mut basis = Vec::new();
        for j in 0..self.cols {
            if basis.len() >= rank { break; }
            let col: Vec<f64> = self.data.iter().map(|row| row[j]).collect();
            // Check linear independence
            let mut test = basis.clone();
            test.push(col.clone());
            if Self::rows_rank(&test) > basis.len() {
                basis.push(col);
            }
        }
        basis
    }

    fn rows_rank(rows: &[Vec<f64>]) -> usize {
        if rows.is_empty() { return 0; }
        let n = rows[0].len();
        let m = rows.len();
        let mut aug: Vec<Vec<f64>> = rows.to_vec();
        let mut pivot_row = 0;
        let mut rank = 0;
        for col in 0..n {
            if pivot_row >= m { break; }
            let mut found = None;
            for row in pivot_row..m {
                if aug[row][col].abs() > 1e-10 {
                    found = Some(row);
                    break;
                }
            }
            if let Some(row) = found {
                aug.swap(pivot_row, row);
                let scale = aug[pivot_row][col];
                for j in 0..n {
                    aug[pivot_row][j] /= scale;
                }
                for row in 0..m {
                    if row != pivot_row && aug[row][col].abs() > 1e-10 {
                        let factor = aug[row][col];
                        for j in 0..n {
                            aug[row][j] -= factor * aug[pivot_row][j];
                        }
                    }
                }
                pivot_row += 1;
                rank += 1;
            }
        }
        rank
    }

    pub fn rank(&self) -> usize {
        Self::rows_rank(&self.data)
    }

    pub fn nullity(&self) -> usize {
        self.cols.saturating_sub(self.rank())
    }
}
