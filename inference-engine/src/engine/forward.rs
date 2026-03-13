use crate::engine::unification;
use crate::types::*;
use crate::types::Term::*;

pub fn forward_chain(kb: &mut KnowledgeBase) -> usize {
    let mut facts = vec![];
    let mut prev = 0;
    let mut res = 1;
    while prev != res {
		prev = res;
        for rule in &kb.rules {
            match find_facts(&kb.facts, rule) {
                f if !f.is_empty() => facts.push(f),
                _ => (),
            }
        }
        for v in facts {
			for fact in v {
				if kb.add_fact(fact) {
					res += 1;
				}
			}
		}
        facts = vec![];
    }
    res - 1
}

fn find_facts(facts: &[Fact], rule: &Rule) -> Vec<Fact> {
    let mut res = vec![];
    for fact in facts {
        match unification::unifier(&rule.head, &fact.term, Substitution::new()) {
            Some(s) => {
                match make_fact(rule, &s) {
					Some(f) => res.push(f),
					_ =>  (),
				}
            }
            _ => (),
        }
    }
    res
}

fn make_fact(rule: &Rule, sub: &Substitution) -> Option<Fact> {
	match make_term(&rule.head, sub) {
		Some(term) => Some(Fact{term}),
		None => None,
	}
}

fn make_term(term: &Term, sub: &Substitution) -> Option<Term> {
    match term {
		Atom(_) => Some(term.clone()),
		Variable(s) => match sub.get(s) {
			Some(n) => Some(Atom(n.clone())),
			None => None,
		}
		Compound{functor, args: a} => {
			let mut args = vec![];
			for arg in a {
				match make_term(arg, sub) {
					Some(t) => args.push(t),
					None => return None,
				}
			}
			Some(Compound{functor: functor.clone(), args})
		}
	}
}
