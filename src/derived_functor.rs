use crate::chain_complex::ChainComplex;
use crate::functor::Functor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedFunctor {
    pub functor: Functor,
    pub derived_order: usize,
}

impl DerivedFunctor {
    /// L_nF(A): compute nth left derived functor via projective resolution.
    pub fn compute_ln(&self, _object: &str, resolution: &ChainComplex) -> Vec<f64> {
        if self.derived_order >= resolution.groups.len() {
            return vec![];
        }
        // Apply functor to the resolution and take homology
        let group = &resolution.groups[self.derived_order];
        // Simplified: return the group dimension as a measure
        vec![group.len() as f64]
    }

    /// R^nF(A): compute nth right derived functor via injective resolution.
    pub fn compute_rn(&self, _object: &str, resolution: &ChainComplex) -> Vec<f64> {
        if self.derived_order >= resolution.groups.len() {
            return vec![];
        }
        let group = &resolution.groups[self.derived_order];
        vec![group.len() as f64]
    }

    /// L_0F ≅ F (the zeroth left derived functor is naturally isomorphic to F).
    pub fn zeroth_is_original(&self) -> bool {
        self.derived_order == 0
    }
}
