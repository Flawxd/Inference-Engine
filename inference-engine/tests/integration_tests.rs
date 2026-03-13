use inference_engine::engine::*;
use inference_engine::types::Term::*;
use inference_engine::types::*;

#[test]
#[ignore]
fn test_engine_creation() {
    let kb = KnowledgeBase::new();
    let _engine = Engine::new(kb);
}

#[test]
fn test_forward_chaining_derives_facts() {
    let mut kb = KnowledgeBase::new();
    let f1 = Fact {
        term: Compound {
            functor: "animal".to_string(),
            args: vec![Atom("cat".to_string())],
        },
    };
    let f2 = Fact {
        term: Compound {
            functor: "fur".to_string(),
            args: vec![Atom("cat".to_string())],
        },
    };
    let f3 = Fact {
        term: Compound {
            functor: "mammal".to_string(),
            args: vec![Atom("cat".to_string())],
        },
    };
	let f4 = Fact {
        term: Compound {
            functor: "animal".to_string(),
            args: vec![Atom("frog".to_string())],
        },
	};
	let f5 = Fact {
        term: Compound {
            functor: "is_son".to_string(),
            args: vec![Atom("Bob".to_string()), Atom("Alice".to_string())],
        },
	};
	let f6 = Fact {
        term: Compound {
            functor: "is_parent".to_string(),
            args: vec![Atom("Alice".to_string()), Atom("Bob".to_string())],
        },
	};
	let f7 = Fact {
        term: Compound {
            functor: "is_mom".to_string(),
            args: vec![Atom("Alice".to_string()), Atom("Bob".to_string())],
        },
	};
	let f8 = Fact {
        term: Compound {
            functor: "male".to_string(),
            args: vec![Atom("Bob".to_string())],
        },
	};
	let f9 = Fact {
        term: Compound {
            functor: "female".to_string(),
            args: vec![Atom("Alice".to_string())],
        },
	};

    let rel1 = Compound {
        functor: "mammal".to_string(),
        args: vec![Variable("X".to_string())],
    };
    let rel2 = Compound {
        functor: "fur".to_string(),
        args: vec![Variable("X".to_string())],
    };
    let rel3 = Compound {
        functor: "animal".to_string(),
        args: vec![Variable("X".to_string())],
    };

    let rel4 = Compound {
        functor: "is_son".to_string(),
        args: vec![Variable("X".to_string()), Variable("Y".to_string())],
    };
    let rel5 = Compound {
        functor: "is_parent".to_string(),
        args: vec![Variable("Y".to_string()), Variable("X".to_string())],
    };
    let rel6 = Compound {
        functor: "male".to_string(),
        args: vec![Variable("X".to_string())],
    };

	let rel7 = Compound {
        functor: "is_mom".to_string(),
        args: vec![Variable("X".to_string()), Variable("Y".to_string())],
    };
    let rel8 = Compound {
        functor: "is_parent".to_string(),
        args: vec![Variable("X".to_string()), Variable("Y".to_string())],
    };
    let rel9 = Compound {
        functor: "female".to_string(),
        args: vec![Variable("X".to_string())],
    };

    let r1 = Rule {
        head: rel1,
        body: vec![rel2, rel3],
    };
	let r2 = Rule {
        head: rel4,
        body: vec![rel5, rel6],
    };
	let r3 = Rule {
        head: rel7,
        body: vec![rel8, rel9],
    };

    kb.add_fact(f1.clone());
    kb.add_fact(f2.clone());
    kb.add_fact(f4.clone());
    kb.add_fact(f6.clone());
    kb.add_fact(f8.clone());
    kb.add_fact(f9.clone());
    kb.add_rule(r1);
    kb.add_rule(r2);
    kb.add_rule(r3);

    let r = forward::forward_chain(&mut kb);
    assert_eq!(kb.facts, vec![f1, f2, f4, f6, f8, f9, f3, f5, f7]);
	assert_eq!(3, r);
}

#[test]
#[ignore]
fn test_backward_chaining_proves_goal() {
    let _kb = KnowledgeBase::new();
}

#[test]
#[ignore]
fn test_load_and_query_rules_file() {}
