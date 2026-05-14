//! Tests de l'exportateur de ruleset.
//!
//! Lancer avec : `cargo test exporter`
//! Demo visuelle : `cargo test demo_exporter -- --nocapture`

use inference_engine::exporter::{
    export_fact, export_knowledge_base, export_knowledge_base_fmt, export_rule, export_term,
    ExportFormat,
};
use inference_engine::types::{Fact, KnowledgeBase, Rule, Term};

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn atom(s: &str) -> Term {
    Term::Atom(s.to_string())
}

fn var(s: &str) -> Term {
    Term::Variable(s.to_string())
}

fn compound(functor: &str, args: Vec<Term>) -> Term {
    Term::Compound { functor: functor.to_string(), args }
}

fn fact(term: Term) -> Fact {
    Fact { term }
}

fn rule(head: Term, body: Vec<Term>) -> Rule {
    Rule { head, body }
}

fn animal_kb() -> KnowledgeBase {
    let mut kb = KnowledgeBase::new();
    kb.add_fact(fact(compound("animal",    vec![atom("chat")])));
    kb.add_fact(fact(compound("a_fourrure", vec![atom("chat")])));
    kb.add_fact(fact(compound("animal",    vec![atom("grenouille")])));
    kb.add_rule(rule(
        compound("mammifere", vec![var("X")]),
        vec![
            compound("animal",     vec![var("X")]),
            compound("a_fourrure", vec![var("X")]),
        ],
    ));
    kb
}

// ─── Tests unitaires : export_term ───────────────────────────────────────────

#[test]
fn test_export_atom() {
    assert_eq!(export_term(&atom("chat")), "chat");
}

#[test]
fn test_export_variable() {
    assert_eq!(export_term(&var("X")), "X");
}

#[test]
fn test_export_compound_no_args() {
    assert_eq!(export_term(&compound("vrai", vec![])), "vrai()");
}

#[test]
fn test_export_compound_one_arg() {
    assert_eq!(
        export_term(&compound("animal", vec![atom("chat")])),
        "animal(chat)"
    );
}

#[test]
fn test_export_compound_two_args() {
    assert_eq!(
        export_term(&compound("parent", vec![atom("alice"), atom("bob")])),
        "parent(alice, bob)"
    );
}

#[test]
fn test_export_compound_with_variable() {
    assert_eq!(
        export_term(&compound("animal", vec![var("X")])),
        "animal(X)"
    );
}

#[test]
fn test_export_nested_compound() {
    let inner = compound("foo", vec![atom("a")]);
    let outer = compound("bar", vec![inner, atom("b")]);
    assert_eq!(export_term(&outer), "bar(foo(a), b)");
}

// ─── Tests unitaires : export_fact ───────────────────────────────────────────

#[test]
fn test_export_fact_simple() {
    assert_eq!(
        export_fact(&fact(compound("animal", vec![atom("chat")]))),
        "animal(chat)."
    );
}

#[test]
fn test_export_fact_atom() {
    assert_eq!(export_fact(&fact(atom("vrai"))), "vrai.");
}

// ─── Tests unitaires : export_rule ───────────────────────────────────────────

#[test]
fn test_export_rule_one_body() {
    let r = rule(
        compound("mammifere", vec![var("X")]),
        vec![compound("animal", vec![var("X")])],
    );
    assert_eq!(export_rule(&r), "mammifere(X) :- animal(X).");
}

#[test]
fn test_export_rule_two_body() {
    let r = rule(
        compound("mammifere", vec![var("X")]),
        vec![
            compound("animal",     vec![var("X")]),
            compound("a_fourrure", vec![var("X")]),
        ],
    );
    assert_eq!(export_rule(&r), "mammifere(X) :- animal(X), a_fourrure(X).");
}

#[test]
fn test_export_rule_multiple_vars() {
    let r = rule(
        compound("grand_parent", vec![var("X"), var("Z")]),
        vec![
            compound("parent", vec![var("X"), var("Y")]),
            compound("parent", vec![var("Y"), var("Z")]),
        ],
    );
    assert_eq!(
        export_rule(&r),
        "grand_parent(X, Z) :- parent(X, Y), parent(Y, Z)."
    );
}

// ─── Tests : export_knowledge_base (Compact) ─────────────────────────────────

#[test]
fn test_export_kb_compact_empty() {
    let kb = KnowledgeBase::new();
    assert_eq!(export_knowledge_base(&kb), "");
}

#[test]
fn test_export_kb_compact_only_facts() {
    let mut kb = KnowledgeBase::new();
    kb.add_fact(fact(compound("animal", vec![atom("chat")])));
    kb.add_fact(fact(compound("animal", vec![atom("chien")])));
    let out = export_knowledge_base(&kb);
    assert_eq!(out, "animal(chat).\nanimal(chien).");
}

#[test]
fn test_export_kb_compact_facts_and_rules() {
    let kb = animal_kb();
    let out = export_knowledge_base(&kb);
    assert!(out.contains("animal(chat)."));
    assert!(out.contains("a_fourrure(chat)."));
    assert!(out.contains("animal(grenouille)."));
    assert!(out.contains("mammifere(X) :- animal(X), a_fourrure(X)."));
}

// ─── Test d'aller-retour : export → parse → export ───────────────────────────

#[test]
fn test_roundtrip_export_then_parse() {
    use inference_engine::parser::parser;

    let kb_original = animal_kb();
    let exported = export_knowledge_base(&kb_original);

    // Le texte exporté doit être re-parseable sans erreur
    let kb_reparsed = parser::parse(&exported);

    // Les faits et règles doivent être identiques
    assert_eq!(kb_original.facts, kb_reparsed.facts);
    assert_eq!(kb_original.rules, kb_reparsed.rules);
}

// ─── Tests : formats Pretty et Prolog ────────────────────────────────────────

#[test]
fn test_export_pretty_contains_header() {
    let kb = animal_kb();
    let out = export_knowledge_base_fmt(&kb, ExportFormat::Pretty);
    assert!(out.contains("% ── Knowledge Base"));
    assert!(out.contains("Facts : 3"));
    assert!(out.contains("Rules : 1"));
    assert!(out.contains("% --- Facts ---"));
    assert!(out.contains("% --- Rules ---"));
}

#[test]
fn test_export_prolog_contains_module() {
    let kb = animal_kb();
    let out = export_knowledge_base_fmt(&kb, ExportFormat::Prolog);
    assert!(out.contains(":- module(knowledge_base, [])."));
    assert!(out.contains("animal(chat)."));
    assert!(out.contains("mammifere(X) :- animal(X), a_fourrure(X)."));
}

#[test]
fn test_export_compact_parseable_by_default() {
    let kb = animal_kb();
    let compact = export_knowledge_base_fmt(&kb, ExportFormat::Compact);
    let pretty  = export_knowledge_base_fmt(&kb, ExportFormat::Pretty);
    // Le compact ne contient pas de commentaires
    assert!(!compact.contains('%'));
    // Le pretty en contient
    assert!(pretty.contains('%'));
}

// ─── Demo visuelle ────────────────────────────────────────────────────────────

#[test]
fn demo_exporter() {
    let kb = animal_kb();

    println!("\n=== FORMAT COMPACT (parse-ready) ===");
    println!("{}", export_knowledge_base(&kb));

    println!("\n=== FORMAT PRETTY (lisible) ===");
    println!("{}", export_knowledge_base_fmt(&kb, ExportFormat::Pretty));

    println!("\n=== FORMAT PROLOG (SWI-Prolog compatible) ===");
    println!("{}", export_knowledge_base_fmt(&kb, ExportFormat::Prolog));

    println!("\n=== TEST ALLER-RETOUR ===");
    use inference_engine::parser::parser;
    let exported   = export_knowledge_base(&kb);
    let kb_reparsed = parser::parse(&exported);
    println!("Original  : {} facts, {} rules", kb.facts.len(), kb.rules.len());
    println!("Reparsed  : {} facts, {} rules", kb_reparsed.facts.len(), kb_reparsed.rules.len());
    println!("Identical : {}", kb.facts == kb_reparsed.facts && kb.rules == kb_reparsed.rules);
}
