pub mod backward;
pub mod forward;
pub mod unification;

use crate::types::{KnowledgeBase, Substitution, Term};

pub struct Engine {
    pub kb: KnowledgeBase,
}

impl Engine {
    pub fn new(kb: KnowledgeBase) -> Self {
        Self { kb }
    }

    pub fn query(&self, _goal: &Term) -> Vec<Substitution> {
        unimplemented!()
    }
}
