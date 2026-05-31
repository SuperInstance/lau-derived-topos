use crate::category::Category;
use crate::internal_logic::InternalLogic;
use crate::subobject_classifier::SubobjectClassifier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topos {
    pub category: Category,
    pub subobject_classifier: SubobjectClassifier,
    pub exponentials: HashMap<String, String>,  // "B,A" -> B^A,
    pub power_objects: HashMap<String, String>,
}

impl Topos {
    pub fn power_object(&self, obj: &str) -> &str {
        self.power_objects.get(obj).map(|s| s.as_str()).unwrap_or("")
    }

    pub fn exponential(&self, a: &str, b: &str) -> &str {
        self.exponentials.get(&format!("{},{}", b, a)).map(|s| s.as_str()).unwrap_or("")
    }

    pub fn internal_logic(&self) -> InternalLogic {
        let terminal = self.category.terminal_object();
        let truth_value_indices: Vec<usize> = match terminal {
            Some(t) => self.category.hom_set(t, &self.subobject_classifier.omega),
            None => vec![],
        };
        InternalLogic {
            topos_category: self.category.clone(),
            truth_value_indices,
            true_idx: self.subobject_classifier.true_morphism,
        }
    }
}
