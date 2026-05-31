use crate::category::Category;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalLogic {
    pub topos_category: Category,
    /// Morphism indices for truth values (global elements 1 → Ω)
    pub truth_value_indices: Vec<usize>,
    /// Index of the "true" morphism
    pub true_idx: usize,
}

impl InternalLogic {
    fn false_idx(&self) -> usize {
        *self.truth_value_indices.iter().find(|&&t| t != self.true_idx).unwrap_or(&self.true_idx)
    }

    pub fn conjunction(&self, p: usize, q: usize) -> usize {
        if p == self.true_idx && q == self.true_idx {
            self.true_idx
        } else {
            self.false_idx()
        }
    }

    pub fn disjunction(&self, p: usize, q: usize) -> usize {
        if p == self.true_idx || q == self.true_idx {
            self.true_idx
        } else {
            self.false_idx()
        }
    }

    pub fn implication(&self, p: usize, q: usize) -> usize {
        if p == self.true_idx && q != self.true_idx {
            self.false_idx()
        } else {
            self.true_idx
        }
    }

    pub fn negation(&self, p: usize) -> usize {
        self.implication(p, self.false_idx())
    }

    pub fn universal(&self, _var: &str, prop: usize) -> usize {
        prop
    }

    pub fn existential(&self, _var: &str, prop: usize) -> usize {
        prop
    }

    pub fn law_of_excluded_middle(&self) -> bool {
        for &i in &self.truth_value_indices {
            let neg = self.negation(i);
            let disj = self.disjunction(i, neg);
            if disj != self.true_idx {
                return false;
            }
        }
        true
    }

    pub fn is_intuitionistic(&self) -> bool {
        !self.law_of_excluded_middle()
    }
}
