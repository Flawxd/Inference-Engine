use std::collections::{HashSet, HashMap};
use crate::types::{Substitution, Term, Rule, KnowledgeBase, ProofTree, Fact};
use crate::engine::unification::{unifier, appliquer_substitution};

pub fn backward_chain(goal: &Term, kb: &KnowledgeBase) -> Option<ProofTree> {
    let mut visited = HashSet::new();
    prove(goal, &kb.facts, &kb.rules, &HashMap::new(), &mut visited)
}

/// Fonction récursive qui tente de prouver un but avec une substitution courante.
fn prove(
    goal: &Term,
    facts: &[Fact],
    rules: &[Rule],
    subst: &Substitution,
    visited: &mut HashSet<Term>,
) -> Option<ProofTree> {
    let goal_instantiated = appliquer_substitution(goal, subst);

    if visited.contains(&goal_instantiated) {
        return None;
    }
    visited.insert(goal_instantiated.clone());

    if facts.iter().any(|f| f.term == goal_instantiated) {
        return Some(ProofTree {
            goal: goal_instantiated,
            subgoals: Vec::new(),
            rule_used: None,
        });
    }

    for rule in rules {
        if let Some(theta) = unifier(&goal_instantiated, &rule.head, Substitution::new()) {
            let mut new_subst = subst.clone();
            for (k, v) in theta {
                new_subst.insert(k, v);
            }

            let mut subgoal_terms = Vec::new();
            for b in &rule.body {
                subgoal_terms.push(appliquer_substitution(b, &new_subst));
            }

            let mut subproofs = Vec::new();
            let mut ok = true;
            let mut local_visited = visited.clone();

            for sg in &subgoal_terms {
                if let Some(subtree) = prove(sg, facts, rules, &new_subst, &mut local_visited) {
                    subproofs.push(subtree);
                } else {
                    ok = false;
                    break;
                }
            }

            if ok {
                visited.extend(local_visited);
                let rule_name = format_rule(rule);
                return Some(ProofTree {
                    goal: goal_instantiated,
                    subgoals: subproofs,
                    rule_used: Some(rule_name),
                });
            }
        }
    }

    None
}

/// Formate une règle en chaîne de caractères pour affichage dans l'arbre de preuve.
fn format_rule(rule: &Rule) -> String {
    let head_str = format_term(&rule.head);
    if rule.body.is_empty() {
        format!("{}.", head_str)
    } else {
        let body_str: Vec<String> = rule.body.iter().map(format_term).collect();
        format!("{} :- {}.", head_str, body_str.join(", "))
    }
}

/// Formate un terme en chaîne de caractères.
fn format_term(term: &Term) -> String {
    match term {
        Term::Atom(c) => c.clone(),
        Term::Variable(v) => v.clone(),
        Term::Compound { functor, args } => {
            if args.is_empty() {
                functor.clone()
            } else {
                let args_str: Vec<String> = args.iter().map(format_term).collect();
                format!("{}({})", functor, args_str.join(", "))
            }
        }
    }
}
