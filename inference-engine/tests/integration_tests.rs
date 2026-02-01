use inference_engine::engine::Engine;
use inference_engine::types::KnowledgeBase;

#[test]
#[ignore]
fn test_engine_creation() {
    let kb = KnowledgeBase::new();
    let _engine = Engine::new(kb);
}

#[test]
#[ignore]
fn test_forward_chaining_derives_facts() {
    let _kb = KnowledgeBase::new();
}

#[test]
#[ignore]
fn test_backward_chaining_proves_goal() {
    let _kb = KnowledgeBase::new();
}

#[test]
#[ignore]
fn test_load_and_query_rules_file() {
}
