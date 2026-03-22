use crate::types::Term::*;
use crate::types::*;
use std::fmt;

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom(name) => write!(f, "{}", name),
            Variable(name) => write!(f, "{}", name),
            Compound { functor, args } => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "{}({})", functor, args_str.join(", "))
            }
        }
    }
}

fn resolve(term: &Term, substitution: &Substitution) -> Term {
    match term {
        Variable(name) => match substitution.get(name) {
            Some(bound) => resolve(&Atom(bound.to_string()), substitution),
            None => term.clone(),
        },
        _ => term.clone(),
    }
}

fn occurs_in(variable: &str, term: &Term, substitution: &Substitution) -> bool {
    match resolve(term, substitution) {
        Atom(_) => false,
        Variable(name) => name == variable,
        Compound { functor: _, args } => args
            .iter()
            .any(|a| occurs_in(variable, a, substitution)),
    }
}

pub fn unify(t1: &Term, t2: &Term, substitution: Substitution) -> Option<Substitution> {
    let mut substitution = substitution;
    unify_mut(t1, t2, &mut substitution)?;
    Some(substitution)
}

fn unify_mut(t1: &Term, t2: &Term, substitution: &mut Substitution) -> Option<()> {
    let t1 = resolve(t1, substitution);
    let t2 = resolve(t2, substitution);
    match (&t1, &t2) {
        (Term::Atom(a), Term::Atom(b)) if a == b => Some(()),
        (Variable(a), Variable(b)) if a == b => Some(()),
        (Variable(name), _) => {
            if occurs_in(name, &t2, substitution) {
                return None;
            }
            substitution.insert(name.clone(), t2.to_string());
            Some(())
        }
        (_, Variable(name)) => {
            if occurs_in(name, &t1, substitution) {
                return None;
            }
            substitution.insert(name.clone(), t1.to_string());
            Some(())
        }
        (
            Compound {
                functor: f1,
                args: args1,
            },
            Compound {
                functor: f2,
                args: args2,
            },
        ) => {
            if f1 != f2 || args1.len() != args2.len() {
                return None;
            }
            for (a1, a2) in args1.iter().zip(args2.iter()) {
                unify_mut(a1, a2, substitution)?;
            }
            Some(())
        }
        _ => None,
    }
}

pub fn apply_substitution(term: &Term, substitution: &Substitution) -> Term {
    match term {
        Atom(_) => term.clone(),
        Variable(name) => match substitution.get(name) {
            Some(bound) => apply_substitution(&Atom(bound.to_string()), substitution),
            None => term.clone(),
        },
        Compound {
            functor,
            args: arguments,
        } => {
            let args = arguments
                .iter()
                .map(|a| apply_substitution(a, substitution))
                .collect();
            Compound {
                functor: functor.clone(),
                args,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_identical_constants() {
        assert!(unify(&cst("cat"), &cst("cat"), Substitution::new()).is_some());
    }
    #[test]
    fn test_different_constants() {
        assert!(unify(&cst("cat"), &cst("dog"), Substitution::new()).is_none());
    }
    #[test]
    fn test_variable_bound_to_constant() {
        let substitution = unify(&var("X"), &cst("cat"), Substitution::new()).unwrap();
        assert_eq!(substitution.get("X"), Some(&"cat".to_string()));
    }
    #[test]
    fn test_compound_simple() {
        let t1 = compound("parent", vec![var("X"), var("Y")]);
        let t2 = compound("parent", vec![cst("alice"), cst("bob")]);
        let substitution = unify(&t1, &t2, Substitution::new()).unwrap();
        assert_eq!(substitution.get("X"), Some("alice".to_string()).as_ref());
        assert_eq!(substitution.get("Y"), Some("bob".to_string()).as_ref());
    }
    #[test]
    fn test_occurs_check() {
        assert!(unify(
            &var("X"),
            &compound("f", vec![var("X")]),
            Substitution::new()
        )
        .is_none());
    }
    #[test]
    fn test_apply_substitution() {
        let term = Compound {
            functor: "mammal".to_string(),
            args: vec![var("X")],
        };
        let mut substitution = Substitution::new();
        substitution.insert("X".to_string(), cst("cat").to_string());
        assert_eq!(
            apply_substitution(&term, &substitution),
            compound("mammal", vec![cst("cat")])
        );
    }
    fn cst(name: &str) -> Term {
        Atom(name.to_string())
    }
    fn var(name: &str) -> Term {
        Variable(name.to_string())
    }
    fn compound(functor: &str, args: Vec<Term>) -> Term {
        Compound {
            functor: functor.to_string(),
            args,
        }
    }
}
