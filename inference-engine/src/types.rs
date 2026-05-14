use std::collections::HashMap;
use std::fmt;

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
		match self.rules.contains(&rule) {
			false => self.rules.push(rule),
			_ => (),
		}
    }
}

impl fmt::Display for Fact {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.term)
	}
}

impl fmt::Display for Rule {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let body = self.body.iter().map(|a| a.to_string()).collect::<Vec<String>>();
		write!(f, "{} :- {}", self.head, body.join(", "))
	}
}

impl fmt::Display for KnowledgeBase {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut res = String::new();
		let facts = self.facts.iter().map(|f| f.to_string()).collect::<Vec<String>>();
		let rules = self.rules.iter().map(|r| r.to_string()).collect::<Vec<String>>();
		if !facts.is_empty() {
			res = format!("Facts: \n\t{}", facts.join("\n\t"));
		}
		if !rules.is_empty() {
			if !res.is_empty() {
				res = format!("{res}\n");
			}
			res = format!("{res}Rules: \n\t{}", rules.join("\n\t"));
		} else if res.is_empty() {
			res = String::from("Empty");
		}
		write!(f, "{}", res)
	}
}
