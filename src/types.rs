use serde::{Deserialize, Serialize};

/// Shared type aliases and simple structs used across modules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphismData {
    pub name: String,
    pub domain: String,
    pub codomain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DenseMatrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}
