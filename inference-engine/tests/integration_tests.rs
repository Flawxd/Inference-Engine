use inference_engine::engine::*;
use inference_engine::parser::parser;
use inference_engine::types::Term::*;
use inference_engine::types::*;

#[test]
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
fn test_backward_chaining_proves_goal() {
    let _kb = KnowledgeBase::new();
}

#[test]
fn test_load_and_query_rules_file() {}

// ── Demo tests ── run with: cargo test demo_ -- --nocapture ──

#[test]
fn demo_parser() {
    let input = "animal(chat). a_fourrure(chat). mammifere(X) :- animal(X), a_fourrure(X).";
    println!("\n=== PARSER DEMO ===");
    println!("Input: {}", input);
    let kb = parser::parse(input);
    println!("Facts:");
    for f in &kb.facts {
        println!("  {:?}", f.term);
    }
    println!("Rules:");
    for r in &kb.rules {
        println!("  {:?} :- {:?}", r.head, r.body);
    }
}

#[test]
fn demo_unification() {
    println!("\n=== UNIFICATION DEMO ===");
    let t1 = Compound { functor: "parent".into(), args: vec![Variable("X".into()), Variable("Y".into())] };
    let t2 = Compound { functor: "parent".into(), args: vec![Atom("alice".into()), Atom("bob".into())] };
    println!("Unify: {:?}", t1);
    println!("With:  {:?}", t2);
    let result = unification::unifier(&t1, &t2, Substitution::new());
    println!("Result: {:?}", result);

    let t3 = Atom("chat".into());
    let t4 = Atom("chien".into());
    println!("\nUnify: {:?} with {:?}", t3, t4);
    println!("Result: {:?}", unification::unifier(&t3, &t4, Substitution::new()));
}

#[test]
fn demo_forward_chaining() {
    println!("\n=== FORWARD CHAINING DEMO ===");
    let mut kb = parser::parse("animal(chat). a_fourrure(chat). animal(grenouille). mammifere(X) :- animal(X), a_fourrure(X).");
    println!("Initial facts:");
    for f in &kb.facts { println!("  {}", f.term); }
    println!("Rules:");
    for r in &kb.rules { println!("  {} :- {}", r.head, r.body.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", ")); }

    let new_facts = forward::forward_chain(&mut kb);
    println!("New facts derived: {}", new_facts);
    println!("All facts after forward chaining:");
    for f in &kb.facts { println!("  {}", f.term); }
}

#[test]
fn demo_backward_chaining() {
    println!("\n=== BACKWARD CHAINING DEMO ===");
    let kb = parser::parse("animal(chat). a_fourrure(chat). mammifere(X) :- animal(X), a_fourrure(X).");
    let goal = Compound { functor: "mammifere".into(), args: vec![Atom("chat".into())] };
    println!("Goal: {}", goal);
    match backward::backward_chain(&goal, &kb) {
        Some(proof) => print_proof(&proof, 0),
        None => println!("  Not provable."),
    }

    let goal2 = Compound { functor: "mammifere".into(), args: vec![Atom("chien".into())] };
    println!("\nGoal: {}", goal2);
    match backward::backward_chain(&goal2, &kb) {
        Some(proof) => print_proof(&proof, 0),
        None => println!("  Not provable."),
    }
}

fn print_proof(proof: &ProofTree, depth: usize) {
    let indent = "  ".repeat(depth + 1);
    match &proof.rule_used {
        Some(rule) => println!("{}Proved: {} (via {})", indent, proof.goal, rule),
        None => println!("{}Proved: {} (fact)", indent, proof.goal),
    }
    for sub in &proof.subgoals {
        print_proof(sub, depth + 1);
    }
}
