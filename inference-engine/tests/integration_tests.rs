use inference_engine::engine::*;
use inference_engine::types::*;
use inference_engine::types::Term::*;

#[test]
#[ignore]
fn test_engine_creation() {
    let kb = KnowledgeBase::new();
    let _engine = Engine::new(kb);
}

#[test]
fn test_forward_chaining_derives_facts() {
    let mut kb = KnowledgeBase::new();
    let f1 = Fact { term: Compound {
		functor: "animal".to_string(),
		args: vec![Atom("cat".to_string())]

	}};
    let f2 = Fact { term: Compound {
        functor: "fur".to_string(),
        args: vec![Atom("cat".to_string())],
    }};
    let f3 = Fact { term: Compound {
        functor: "mammal".to_string(),
        args: vec![Atom("cat".to_string())],
    }};

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

    let r1 = Rule {
        head: rel1,
        body: vec![rel2, rel3],
    };

    kb.add_fact(f1.clone());
    kb.add_fact(f2.clone());
    kb.add_rule(r1);

    forward::forward_chain(&mut kb);
    assert_eq!(kb.facts, vec![f1, f2, f3]);
}

#[test]
#[ignore]
fn test_backward_chaining_proves_goal() {
    let _kb = KnowledgeBase::new();
}

#[test]
#[ignore]
fn test_load_and_query_rules_file() {}
