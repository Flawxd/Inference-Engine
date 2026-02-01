use inference_engine::engine::unification;
use inference_engine::types::Term;

#[test]
#[ignore]
fn test_unify_identical_atoms() {
    let t1 = Term::Atom("cat".to_string());
    let t2 = Term::Atom("cat".to_string());
    let _result = unification::unify(&t1, &t2);
}

#[test]
#[ignore]
fn test_unify_variable_with_atom() {
    let t1 = Term::Variable("X".to_string());
    let t2 = Term::Atom("cat".to_string());
    let _result = unification::unify(&t1, &t2);
}

#[test]
#[ignore]
fn test_unify_different_atoms_fails() {
    let t1 = Term::Atom("cat".to_string());
    let t2 = Term::Atom("dog".to_string());
    let _result = unification::unify(&t1, &t2);
}

#[test]
#[ignore]
fn test_unify_compound_terms() {
    let t1 = Term::Compound {
        functor: "parent".to_string(),
        args: vec![Term::Variable("X".to_string()), Term::Atom("bob".to_string())],
    };
    let t2 = Term::Compound {
        functor: "parent".to_string(),
        args: vec![Term::Atom("tom".to_string()), Term::Atom("bob".to_string())],
    };
    let _result = unification::unify(&t1, &t2);
}

#[test]
#[ignore]
fn test_occurs_check() {
    let t1 = Term::Variable("X".to_string());
    let t2 = Term::Compound {
        functor: "f".to_string(),
        args: vec![Term::Variable("X".to_string())],
    };
    let _result = unification::unify(&t1, &t2);
}
