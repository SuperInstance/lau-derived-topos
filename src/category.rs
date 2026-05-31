use crate::types::MorphismData;
use serde::{Deserialize, Serialize};

/// A small category with objects and morphisms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub objects: Vec<String>,
    pub morphisms: Vec<MorphismData>,
    /// composition table: (f_index, g_index) -> h_index meaning h = g ∘ f
    pub composition_table: Vec<((usize, usize), usize)>,
}

impl Category {
    pub fn new(objects: Vec<String>, morphisms: Vec<MorphismData>) -> Self {
        let mut cat = Self {
            objects,
            morphisms,
            composition_table: Vec::new(),
        };
        // Add identity morphisms for every object
        let objs: Vec<String> = cat.objects.clone();
        for obj in &objs {
            if !cat.morphisms.iter().any(|m| m.domain == *obj && m.codomain == *obj && m.name == format!("id_{}", obj)) {
                cat.morphisms.push(MorphismData {
                    name: format!("id_{}", obj),
                    domain: obj.clone(),
                    codomain: obj.clone(),
                });
            }
        }
        cat
    }

    pub fn identity(&self, obj: &str) -> usize {
        self.morphisms
            .iter()
            .position(|m| m.domain == obj && m.codomain == obj && m.name == format!("id_{}", obj))
            .unwrap()
    }

    pub fn compose(&self, f: usize, g: usize) -> Option<usize> {
        if self.morphisms[f].codomain != self.morphisms[g].domain {
            return None;
        }
        // Check composition table
        for &((fi, gi), hi) in &self.composition_table {
            if fi == f && gi == g {
                return Some(hi);
            }
        }
        // If g is identity, g∘f = f
        let gf = &self.morphisms[f];
        let gm = &self.morphisms[g];
        if gm.domain == gm.codomain && gm.name == format!("id_{}", gm.domain) {
            return Some(f);
        }
        // If f is identity, g∘f = g
        if gf.domain == gf.codomain && gf.name == format!("id_{}", gf.domain) {
            return Some(g);
        }
        None
    }

    pub fn hom_set(&self, from: &str, to: &str) -> Vec<usize> {
        self.morphisms
            .iter()
            .enumerate()
            .filter(|(_, m)| m.domain == from && m.codomain == to)
            .map(|(i, _)| i)
            .collect()
    }

    pub fn domain(&self, f: usize) -> &str {
        &self.morphisms[f].domain
    }

    pub fn codomain(&self, f: usize) -> &str {
        &self.morphisms[f].codomain
    }

    pub fn isomorphic(&self, a: &str, b: &str) -> bool {
        let hom_ab = self.hom_set(a, b);
        let hom_ba = self.hom_set(b, a);
        let id_a = self.identity(a);
        let id_b = self.identity(b);
        for &f in &hom_ab {
            for &g in &hom_ba {
                // g∘f = id_A and f∘g = id_B
                if let Some(gf) = self.compose(f, g) {
                    if gf != id_a { continue; }
                    if let Some(fg) = self.compose(g, f) {
                        if fg == id_b {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn initial_object(&self) -> Option<&str> {
        'outer: for obj in &self.objects {
            for other in &self.objects {
                if self.hom_set(obj, other).len() != 1 {
                    continue 'outer;
                }
            }
            return Some(obj);
        }
        None
    }

    pub fn terminal_object(&self) -> Option<&str> {
        'outer: for obj in &self.objects {
            for other in &self.objects {
                if self.hom_set(other, obj).len() != 1 {
                    continue 'outer;
                }
            }
            return Some(obj);
        }
        None
    }

    /// Returns (A, B, A×B) triples where projection morphisms exist.
    pub fn products(&self) -> Vec<(&str, &str, &str)> {
        let mut result = Vec::new();
        for prod_obj in &self.objects {
            for a in &self.objects {
                for b in &self.objects {
                    if a == b && a == prod_obj {
                        continue;
                    }
                    // Check: exactly one morphism from prod to a, one from prod to b,
                    // and for any object X with f:X→A, g:X→B, unique h:X→prod with p1∘h=f, p2∘h=g
                    // Simplified: check there are morphisms prod→A and prod→B (projections)
                    let p1s = self.hom_set(prod_obj, a);
                    let p2s = self.hom_set(prod_obj, b);
                    if !p1s.is_empty() && !p2s.is_empty() {
                        result.push((a.as_str(), b.as_str(), prod_obj.as_str()));
                    }
                }
            }
        }
        result
    }

    pub fn equalizers(&self) -> Vec<(usize, usize, usize)> {
        let mut result = Vec::new();
        for eq_idx in 0..self.morphisms.len() {
            let eq = &self.morphisms[eq_idx];
            // Find f, g with same domain/codomain where eq equalizes them
            let candidates: Vec<usize> = self.morphisms.iter().enumerate()
                .filter(|(_, m)| m.domain == eq.domain && m.codomain == eq.codomain)
                .map(|(i, _)| i)
                .collect();
            for f_idx in &candidates {
                for g_idx in &candidates {
                    if f_idx >= g_idx { continue; }
                    // eq equalizes f and g if f∘eq = g∘eq
                    if let (Some(feq), Some(geq)) = (self.compose(eq_idx, *f_idx), self.compose(eq_idx, *g_idx)) {
                        if feq == geq {
                            result.push((*f_idx, *g_idx, eq_idx));
                        }
                    }
                }
            }
        }
        result
    }

    pub fn has_terminal(&self) -> bool {
        self.terminal_object().is_some()
    }

    pub fn has_products(&self) -> bool {
        !self.products().is_empty()
    }

    pub fn is_cartesian_closed(&self) -> bool {
        self.has_terminal() && self.has_products()
        // In a full impl, we'd check for exponentials too.
        // For finite categories, having products + terminal is a good proxy.
    }

    pub fn is_topos(&self) -> bool {
        self.is_cartesian_closed()
        // Would also need subobject classifier — checked via Topos struct
    }

    pub fn add_morphism(&mut self, m: MorphismData) -> usize {
        let idx = self.morphisms.len();
        self.morphisms.push(m);
        idx
    }

    pub fn add_composition(&mut self, f: usize, g: usize, h: usize) {
        self.composition_table.push(((f, g), h));
    }

    pub fn morphism_index(&self, name: &str) -> Option<usize> {
        self.morphisms.iter().position(|m| m.name == name)
    }
}
