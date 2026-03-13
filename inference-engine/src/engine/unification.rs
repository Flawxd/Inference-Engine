use std::fmt;
use crate::types::*;
use crate::types::Term::*;

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom(nom) => write!(f, "{}", nom),
            Variable(nom) => write!(f, "{}", nom),
            Compound{functor, args} => {
                let chaine_args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "{}({})", functor, chaine_args.join(", "))
            }
        }
    }
}

fn resoudre(terme: &Term, substitution: &Substitution) -> Term {
    match terme {
        Variable(nom) => match substitution.get(nom) {
            Some(lie) => resoudre(&Variable(lie.to_string()), substitution),
            None => terme.clone(),
        },
        _ => terme.clone(),
    }
}

fn apparait_dans(variable: &str, terme: &Term, substitution: &Substitution) -> bool {
    match resoudre(terme, substitution) {
        Atom(_) => false,
        Variable(nom) => nom == variable,
        Compound{functor: _, args} => args
            .iter()
            .any(|a| apparait_dans(variable, a, substitution)),
    }
}

pub fn unifier(t1: &Term, t2: &Term, substitution: Substitution) -> Option<Substitution> {
    let mut substitution = substitution;
    unifier_mut(t1, t2, &mut substitution)?;
    Some(substitution)
}

fn unifier_mut(t1: &Term, t2: &Term, substitution: &mut Substitution) -> Option<()> {
    let t1 = resoudre(t1, substitution);
    let t2 = resoudre(t2, substitution);
    match (&t1, &t2) {
        (Term::Atom(a), Term::Atom(b)) if a == b => Some(()),
        (Variable(a), Variable(b)) if a == b => Some(()),
        (Variable(nom), _) => {
            if apparait_dans(nom, &t2, substitution) {
                return None;
            }
            substitution.insert(nom.clone(), t2.to_string());
            Some(())
        }
        (_, Variable(nom)) => {
            if apparait_dans(nom, &t1, substitution) {
                return None;
            }
            substitution.insert(nom.clone(), t1.to_string());
            Some(())
        }
        (Compound{functor: f1, args: args1}, Compound{functor: f2, args: args2}) => {
            if f1 != f2 || args1.len() != args2.len() {
                return None;
            }
            for (a1, a2) in args1.iter().zip(args2.iter()) {
                unifier_mut(a1, a2, substitution)?;
            }
            Some(())
        }
        _ => None,
    }
}

pub fn appliquer_substitution(terme: &Term, substitution: &Substitution) -> Term {
    match terme {
        Atom(_) => terme.clone(),
        Variable(nom) => match substitution.get(nom) {
            Some(lie) => appliquer_substitution(&Variable(lie.to_string()), substitution),
            None => terme.clone(),
        },
        Compound{functor, args: arguments} => {
            let args = arguments
                .iter()
                .map(|a| appliquer_substitution(a, substitution))
                .collect();
            Compound {functor: functor.clone(), args: args}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_constantes_identiques() {
        assert!(unifier(&cst("chat"), &cst("chat"), Substitution::new()).is_some());
    }
    #[test]
    fn test_constantes_differentes() {
        assert!(unifier(&cst("chat"), &cst("chien"), Substitution::new()).is_none());
    }
    #[test]
    fn test_variable_liee_a_constante() {
        let substitution = unifier(&var("X"), &cst("chat"), Substitution::new()).unwrap();
        assert_eq!(substitution.get("X"), Some(&"chat".to_string()));
    }
    #[test]
    fn test_compose_simple() {
        let t1 = compose("parent", vec![var("X"), var("Y")]);
        let t2 = compose("parent", vec![cst("alice"), cst("bob")]);
        let substitution = unifier(&t1, &t2, Substitution::new()).unwrap();
        assert_eq!(substitution.get("X"), Some("alice".to_string()).as_ref());
        assert_eq!(substitution.get("Y"), Some("bob".to_string()).as_ref());
    }
    #[test]
    fn test_verification_occurrence() {
        assert!(unifier(&var("X"), &compose("f", vec![var("X")]), Substitution::new()).is_none());
    }
    #[test]
    fn test_appliquer_substitution() {
        let terme = Compound{functor: "mammifere".to_string(), args: vec![var("X")]};
        let mut substitution = Substitution::new();
        substitution.insert("X".to_string(), cst("chat").to_string());
        assert_eq!(
            appliquer_substitution(&terme, &substitution),
            compose("mammifere", vec![cst("chat")])
        );
    }
    fn cst(nom: &str) -> Term {
        Atom(nom.to_string())
    }
    fn var(nom: &str) -> Term {
        Variable(nom.to_string())
    }
    fn compose(foncteur: &str, args: Vec<Term>) -> Term {
        Compound{functor: foncteur.to_string(), args}
    }
}
