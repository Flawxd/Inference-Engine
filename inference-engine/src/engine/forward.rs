use crate::engine::unification::{appliquer_substitution, unifier};
use crate::types::*;

pub fn forward_chain(kb: &mut KnowledgeBase) -> usize {
    let mut facts = vec![];
    let mut prev = 0;
    let mut res = 1;
    while prev != res {
        prev = res;
        for rule in &kb.rules {
            for sub in find_subs(&kb.facts, rule) {
                facts.push( Fact {
					term: appliquer_substitution(&rule.head, &sub)
				});
			}
        }
        for fact in facts {
			if kb.add_fact(fact) {
                res += 1;
            }
        }
        facts = vec![];
    }
    res - 1
}

fn find_subs(facts: &[Fact], rule: &Rule) -> Vec<Substitution> {
    let mut res = vec![];
	let mut new = vec![];
    for term in &rule.body {
        for sub in equate(&term, facts) {
            new.push(sub);
        }
        res = if res.is_empty() {
			new
		} else {
			update_subs(&res, &new)
		};
		new = vec![];
    }
    res
}

fn equate(term: &Term, facts: &[Fact]) -> Vec<Substitution> {
	let mut res = vec![];
	for fact in facts {
		match unifier(term, &fact.term, Substitution::new()) {
			Some(s) => res.push(s),
			_ => (),
		}
	}
	res
}

fn subs_match(s1: &Substitution, s2: &Substitution) -> Option<Substitution> {
	let mut res = s1.clone();
	for (var, atom) in s2.iter() {
		match s1.get(var) {
			Some(a) if a == atom => (),
			None => {res.insert(var.to_string(), atom.to_string());},
			_  => return None,
		}
	}
	Some(res)
}

fn update_subs(old: &[Substitution], new: &[Substitution]) -> Vec<Substitution> {
	let mut res = vec![];
	for base in old {
		for add in new {
			match subs_match(&base, &add) {
				Some(s) => res.push(s),
				_ => (),
			}
		}
	}
	res
}

