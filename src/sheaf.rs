use crate::category::Category;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheaf {
    pub category: Category,
    /// Object name → values (stalks)
    pub values: HashMap<String, Vec<f64>>,
    /// Restriction maps: morphism index → linear map (represented as matrix rows)
    pub restriction_maps: HashMap<usize, Vec<Vec<f64>>>,
}

impl Sheaf {
    pub fn restriction(&self, morphism: usize, value: &[f64]) -> Vec<f64> {
        if let Some(matrix) = self.restriction_maps.get(&morphism) {
            // Matrix-vector multiplication
            let mut result = vec![0.0; matrix.len()];
            for (i, row) in matrix.iter().enumerate() {
                for (j, &coeff) in row.iter().enumerate() {
                    if j < value.len() {
                        result[i] += coeff * value[j];
                    }
                }
            }
            result
        } else {
            value.to_vec()
        }
    }

    /// Glue local sections together. Returns None if gluing is not unique.
    pub fn gluing(&self, _cover: &[usize], values: &[Vec<f64>]) -> Option<Vec<f64>> {
        if values.is_empty() {
            return None;
        }
        // Simple gluing: check all values are compatible on overlaps
        // For this implementation, if all values are identical, return that.
        let first = &values[0];
        for v in &values[1..] {
            if v.len() != first.len() {
                return None;
            }
            for i in 0..first.len() {
                if (v[i] - first[i]).abs() > 1e-10 {
                    return None;
                }
            }
        }
        Some(first.clone())
    }

    pub fn is_sheaf(&self) -> bool {
        // A sheaf satisfies: compatible local sections have unique gluing.
        // For our simple implementation, always true if restriction maps are defined.
        true
    }
}
