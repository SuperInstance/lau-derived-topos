use crate::functor::Functor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalTransformation {
    pub source: Functor,
    pub target: Functor,
    /// object name → morphism index in target category
    pub components: HashMap<String, usize>,
}

impl NaturalTransformation {
    /// Check that β_B ∘ F(f) = G(f) ∘ β_A for a given f.
    pub fn naturality_square(&self, f: usize) -> bool {
        let src_cat = &self.source.source;
        let tgt_cat = &self.target.target;

        let f_dom = src_cat.domain(f).to_string();
        let f_cod = src_cat.codomain(f).to_string();

        let f_mapped_src = self.source.on_morphisms(f);
        let f_mapped_tgt = self.target.on_morphisms(f);

        let beta_a = *self.components.get(&f_dom).unwrap_or(&0);
        let beta_b = *self.components.get(&f_cod).unwrap_or(&0);

        // β_B ∘ F(f)
        let left = tgt_cat.compose(f_mapped_src, beta_b);
        // G(f) ∘ β_A
        let right = tgt_cat.compose(beta_a, f_mapped_tgt);

        match (left, right) {
            (Some(l), Some(r)) => l == r,
            _ => false,
        }
    }

    pub fn is_natural_isomorphism(&self) -> bool {
        self.components.values().all(|&comp| {
            let m = &self.target.target.morphisms[comp];
            // Check if component is iso by looking for inverse
            let inverses = self.target.target.hom_set(&m.codomain, &m.domain);
            for &inv in &inverses {
                if let Some(comp_inv) = self.target.target.compose(comp, inv) {
                    if self.target.target.domain(comp_inv) == m.domain
                        && self.target.target.codomain(comp_inv) == m.domain
                        && comp_inv == self.target.target.identity(&m.domain)
                    {
                        if let Some(inv_comp) = self.target.target.compose(inv, comp) {
                            if inv_comp == self.target.target.identity(&m.codomain) {
                                return true;
                            }
                        }
                    }
                }
            }
            false
        })
    }
}
