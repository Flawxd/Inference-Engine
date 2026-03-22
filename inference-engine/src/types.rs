use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    Atom(String),
    Variable(String),
    Compound { functor: String, args: Vec<Term> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub head: Term,
    pub body: Vec<Term>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fact {
    pub term: Term,
}

pub type Substitution = HashMap<String, String>;

#[derive(Debug, Clone, PartialEq)]
pub struct KnowledgeBase {
    pub facts: Vec<Fact>,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProofTree {
    pub goal: Term,
    pub subgoals: Vec<ProofTree>,
    pub rule_used: Option<String>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self {
            facts: vec![],
            rules: vec![],
        }
    }
    pub fn add_fact(&mut self, fact: Fact) -> bool {
        match self.facts.contains(&fact) {
            false => {
                self.facts.push(fact);
                true
            }
            _ => false,
        }
    }
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }
}
