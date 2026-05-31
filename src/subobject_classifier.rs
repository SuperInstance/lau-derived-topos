use crate::category::Category;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubobjectClassifier {
    pub omega: String,
    /// Index of the true morphism 1 → Ω
    pub true_morphism: usize,
    /// Map from subobject morphism index to characteristic morphism index
    pub characteristic_map: Vec<(usize, usize)>,
    pub category: Category,
}

impl SubobjectClassifier {
    pub fn characteristic(&self, subobject: usize) -> usize {
        self.characteristic_map
            .iter()
            .find(|(s, _)| *s == subobject)
            .map(|(_, c)| *c)
            .unwrap_or(self.true_morphism)
    }

    pub fn pullback(&self, chi: usize) -> usize {
        self.characteristic_map
            .iter()
            .find(|(_, c)| *c == chi)
            .map(|(s, _)| *s)
            .unwrap_or(self.true_morphism)
    }

    /// Global elements of Ω: morphisms from terminal object to Ω.
    pub fn truth_values(&self) -> Vec<String> {
        let terminal = self.category.terminal_object();
        match terminal {
            Some(t) => {
                self.category
                    .hom_set(t, &self.omega)
                    .iter()
                    .map(|&i| self.category.morphisms[i].name.clone())
                    .collect()
            }
            None => vec![],
        }
    }
}
