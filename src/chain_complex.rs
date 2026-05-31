use crate::dense_matrix::DenseMatrix;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainComplex {
    /// C_n at each degree
    pub groups: Vec<Vec<f64>>,
    /// ∂_n: C_n → C_{n-1} (index n corresponds to ∂_n)
    pub differentials: Vec<DenseMatrix>,
}

impl ChainComplex {
    pub fn new(groups: Vec<Vec<f64>>, differentials: Vec<DenseMatrix>) -> Self {
        Self { groups, differentials }
    }

    pub fn boundary(&self, degree: usize) -> &DenseMatrix {
        &self.differentials[degree]
    }

    /// ker(∂_n) — basis vectors for the cycle group
    pub fn cycle(&self, degree: usize) -> Vec<Vec<f64>> {
        if degree >= self.differentials.len() {
            return vec![];
        }
        self.differentials[degree].kernel_basis()
    }

    /// im(∂_{n+1}) — basis vectors for the boundary group
    pub fn boundary_group(&self, degree: usize) -> Vec<Vec<f64>> {
        if degree + 1 >= self.differentials.len() {
            return vec![];
        }
        self.differentials[degree + 1].image_basis()
    }

    /// Homology dimension: dim(ker(∂_n)) - dim(im(∂_{n+1}))
    pub fn homology(&self, degree: usize) -> usize {
        let cycle_dim = if degree < self.differentials.len() {
            self.differentials[degree].kernel_basis().len()
        } else {
            self.groups.get(degree).map(|g| g.len()).unwrap_or(0)
        };
        let boundary_dim = if degree + 1 < self.differentials.len() {
            self.differentials[degree + 1].rank()
        } else {
            0
        };
        cycle_dim.saturating_sub(boundary_dim)
    }

    /// A chain complex is exact if all homology groups are trivial.
    pub fn is_exact(&self) -> bool {
        for i in 0..self.differentials.len().saturating_sub(1) {
            if self.homology(i) != 0 {
                return false;
            }
        }
        true
    }

    /// Verify ∂² = 0: the boundary of boundary is zero.
    pub fn verify_boundary_squared_zero(&self) -> bool {
        for i in 1..self.differentials.len() {
            let product = self.differentials[i].multiply(&self.differentials[i - 1]);
            if product.data.iter().any(|row| row.iter().any(|&x| x.abs() > 1e-10)) {
                return false;
            }
        }
        true
    }
}
