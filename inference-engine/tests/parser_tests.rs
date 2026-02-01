use inference_engine::parser;

#[test]
#[ignore]
fn test_parse_fact() {
    let _result = parser::parse_program("cat(tom).");
}

#[test]
#[ignore]
fn test_parse_rule() {
    let _result = parser::parse_program("mortal(X) :- human(X).");
}

#[test]
#[ignore]
fn test_parse_multiple_statements() {
    let input = "cat(tom). dog(rex). animal(X) :- cat(X).";
    let _result = parser::parse_program(input);
}
