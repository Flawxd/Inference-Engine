use inference_engine::engine::unification;
use inference_engine::types::Term::*;
use inference_engine::types::*;

#[test]
#[ignore]
fn test_unify_identical_atoms() {
    let t1 = Atom("cat".to_string());
    let t2 = Atom("cat".to_string());
    let _result = unification::unifier(&t1, &t2, Substitution::new());
}

#[test]
#[ignore]
fn test_unify_variable_with_atom() {
    let t1 = Variable("X".to_string());
    let t2 = Atom("cat".to_string());
    let _result = unification::unifier(&t1, &t2, Substitution::new());
}

#[test]
#[ignore]
fn test_unify_different_atoms_fails() {
    let t1 = Atom("cat".to_string());
    let t2 = Atom("dog".to_string());
    let _result = unification::unifier(&t1, &t2, Substitution::new());
}

#[test]
#[ignore]
fn test_unify_compound_terms() {
    let t1 = Compound {
        functor: "parent".to_string(),
        args: vec![Variable("X".to_string()), Atom("bob".to_string())],
    };
    let t2 = Compound {
        functor: "parent".to_string(),
        args: vec![Atom("tom".to_string()), Atom("bob".to_string())],
    };
    let _result = unification::unifier(&t1, &t2, Substitution::new());
}

#[test]
#[ignore]
fn test_occurs_check() {
    let t1 = Variable("X".to_string());
    let t2 = Compound {
        functor: "f".to_string(),
        args: vec![Variable("X".to_string())],
    };
    let _result = unification::unifier(&t1, &t2, Substitution::new());
}
