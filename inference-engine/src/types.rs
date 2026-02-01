use std::collections::HashMap;

pub enum Term {
    Atom(String),
    Variable(String),
    Compound { functor: String, args: Vec<Term> },
}

pub struct Rule {
    pub head: Term,
    pub body: Vec<Term>,
}

pub struct Fact {
    pub term: Term,
}

pub type Substitution = HashMap<String, Term>;

pub struct KnowledgeBase {
    pub facts: Vec<Fact>,
    pub rules: Vec<Rule>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_fact(&mut self, _fact: Fact) {
        self.facts.push(fact);
    }
    pub fn add_rule(&mut self, _rule: Rule) {
        self.rules.push(rule);
    }
}
