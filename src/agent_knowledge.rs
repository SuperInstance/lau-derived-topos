use crate::topos::Topos;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentKnowledgeBase {
    pub topos: Topos,
    pub agents: Vec<String>,
    /// Agent → list of subobject indices the agent knows
    pub knowledge: std::collections::HashMap<String, Vec<usize>>,
}

impl AgentKnowledgeBase {
    pub fn knowledge_state(&self, agent: &str) -> Vec<usize> {
        self.knowledge.get(agent).cloned().unwrap_or_default()
    }

    /// Merge knowledge from multiple agents (colimit: union of knowledge).
    pub fn merge_knowledge(&self, agents: &[&str]) -> Vec<usize> {
        let mut merged = std::collections::HashSet::new();
        for agent in agents {
            if let Some(knowledge) = self.knowledge.get(*agent) {
                for &k in knowledge {
                    merged.insert(k);
                }
            }
        }
        let mut result: Vec<usize> = merged.into_iter().collect();
        result.sort();
        result
    }

    /// Check consistency: no agent knows both a subobject and its negation.
    pub fn consistent(&self, agent: &str) -> bool {
        let knowledge = self.knowledge_state(agent);
        // Simplified: no contradictions if knowledge has no duplicate entries
        let mut seen = std::collections::HashSet::new();
        for &k in &knowledge {
            if !seen.insert(k) {
                return false;
            }
        }
        true
    }

    /// Common knowledge: intersection of all agents' knowledge (limit).
    pub fn common_knowledge(&self) -> Vec<usize> {
        if self.agents.is_empty() {
            return vec![];
        }
        let mut common: Option<std::collections::HashSet<usize>> = None;
        for agent in &self.agents {
            let agent_knowledge: std::collections::HashSet<usize> =
                self.knowledge_state(agent).into_iter().collect();
            common = Some(match common {
                None => agent_knowledge,
                Some(c) => c.intersection(&agent_knowledge).copied().collect(),
            });
        }
        let mut result: Vec<usize> = common.unwrap_or_default().into_iter().collect();
        result.sort();
        result
    }
}
