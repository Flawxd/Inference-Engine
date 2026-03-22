use inference_engine::parser::*;

#[test]
fn test_parse_fact() {
    let _result = parser::parse("cat(tom).");
}

#[test]
fn test_parse_rule() {
    let _result = parser::parse("mortal(X) :- human(X).");
}

#[test]
fn test_parse_multiple_statements() {
    let input = "cat(tom). dog(rex). animal(X) :- cat(X).";
    let _result = parser::parse(input);
}
