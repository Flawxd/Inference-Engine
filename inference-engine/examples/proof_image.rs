use inference_engine::engine::backward::backward_chain;
use inference_engine::parser::parser;
use inference_engine::types::Term::*;
use inference_engine::visualize::proof_tree_to_png;

fn main() {
    let kb = parser::parse(
        "animal(chat). a_fourrure(chat). mammifere(X) :- animal(X), a_fourrure(X).",
    );
    let goal = Compound {
        functor: "mammifere".into(),
        args: vec![Atom("chat".into())],
    };
    let tree = backward_chain(&goal, &kb).expect("failed to prove goal");
    proof_tree_to_png(&tree, "proof.png").expect("failed to generate png");
    println!("Saved proof.png");
}
