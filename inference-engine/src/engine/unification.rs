use crate::types::{Substitution, Term};

pub fn unify(_t1: &Term, _t2: &Term) -> Option<Substitution> {
    unimplemented!()
}

pub fn apply_substitution(_term: &Term, _subst: &Substitution) -> Term {
    unimplemented!()
}

pub fn compose(_s1: &Substitution, _s2: &Substitution) -> Substitution {
    unimplemented!()
}

pub fn occurs_check(_var: &str, _term: &Term) -> bool {
    unimplemented!()
}
