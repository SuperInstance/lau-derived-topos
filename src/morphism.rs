use crate::category::Category;
use crate::types::MorphismData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Morphism {
    pub data: MorphismData,
}

impl Morphism {
    pub fn new(name: &str, domain: &str, codomain: &str) -> Self {
        Self {
            data: MorphismData {
                name: name.to_string(),
                domain: domain.to_string(),
                codomain: codomain.to_string(),
            },
        }
    }

    /// Left-cancellative: f∘h = g∘h implies f = g (h is epi-like; this checks mono)
    /// Mono: for all g1, g2: f∘g1 = f∘g2 => g1 = g2
    pub fn is_mono(&self, cat: &Category, f_idx: usize) -> bool {
        let dom = &self.data.domain;
        // Find all morphisms that can compose with f (codomain = f's domain)
        let candidates: Vec<usize> = cat.morphisms.iter().enumerate()
            .filter(|(_, m)| m.codomain == *dom)
            .map(|(i, _)| i)
            .collect();
        for &g1 in &candidates {
            for &g2 in &candidates {
                if g1 == g2 { continue; }
                match (cat.compose(g1, f_idx), cat.compose(g2, f_idx)) {
                    (Some(r1), Some(r2)) if r1 == r2 => return false,
                    _ => {}
                }
            }
        }
        true
    }

    /// Right-cancellative: h∘f = h∘g implies f = g
    pub fn is_epi(&self, cat: &Category, f_idx: usize) -> bool {
        let cod = &self.data.codomain;
        let candidates: Vec<usize> = cat.morphisms.iter().enumerate()
            .filter(|(_, m)| m.domain == *cod)
            .map(|(i, _)| i)
            .collect();
        for &g1 in &candidates {
            for &g2 in &candidates {
                if g1 == g2 { continue; }
                match (cat.compose(f_idx, g1), cat.compose(f_idx, g2)) {
                    (Some(r1), Some(r2)) if r1 == r2 => return false,
                    _ => {}
                }
            }
        }
        true
    }

    pub fn is_iso(&self, cat: &Category, f_idx: usize) -> bool {
        self.is_mono(cat, f_idx) && self.is_epi(cat, f_idx)
    }
}
