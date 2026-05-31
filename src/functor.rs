use crate::category::Category;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Functor {
    pub source: Category,
    pub target: Category,
    pub object_map: HashMap<String, String>,
    pub morphism_map: HashMap<usize, usize>,
}

impl Functor {
    pub fn on_objects<'a>(&'a self, obj: &'a str) -> &'a str {
        self.object_map.get(obj).map(|s| s.as_str()).unwrap_or(obj)
    }

    pub fn on_morphisms(&self, f: usize) -> usize {
        *self.morphism_map.get(&f).unwrap_or(&f)
    }

    pub fn preserves_identities(&self) -> bool {
        for obj in &self.source.objects {
            let id_src = self.source.identity(obj);
            let mapped_id = self.on_morphisms(id_src);
            let target_obj = self.on_objects(obj);
            let id_tgt = self.target.identity(target_obj);
            if mapped_id != id_tgt {
                return false;
            }
        }
        true
    }

    pub fn preserves_composition(&self) -> bool {
        for &((f, g), h) in &self.source.composition_table {
            let fh = self.on_morphisms(f);
            let gh = self.on_morphisms(g);
            let hh = self.on_morphisms(h);
            match self.target.compose(fh, gh) {
                Some(result) if result == hh => {}
                _ => return false,
            }
        }
        true
    }

    pub fn is_full(&self) -> bool {
        for src_obj in &self.source.objects {
            for tgt_obj in &self.source.objects {
                let tgt_src = self.on_objects(src_obj);
                let tgt_tgt = self.on_objects(tgt_obj);
                let target_homs: Vec<usize> = self.target.hom_set(tgt_src, tgt_tgt);
                let source_homs: Vec<usize> = self.source.hom_set(src_obj, tgt_obj);
                let mapped: std::collections::HashSet<usize> = source_homs.iter()
                    .map(|&f| self.on_morphisms(f))
                    .collect();
                if mapped.len() < target_homs.len() {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_faithful(&self) -> bool {
        for src_obj in &self.source.objects {
            for tgt_obj in &self.source.objects {
                let source_homs: Vec<usize> = self.source.hom_set(src_obj, tgt_obj);
                let mapped: Vec<usize> = source_homs.iter().map(|&f| self.on_morphisms(f)).collect();
                // Check injectivity
                let unique: std::collections::HashSet<usize> = mapped.iter().copied().collect();
                if unique.len() != mapped.len() {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_essentially_surjective(&self) -> bool {
        for obj in &self.target.objects {
            let found = self.object_map.values().any(|v| v == obj);
            if !found {
                return false;
            }
        }
        true
    }

    pub fn is_equivalence(&self) -> bool {
        self.is_full() && self.is_faithful() && self.is_essentially_surjective()
    }
}
