use graphrite_core::parser::Parser;

#[test]
fn accepts_valid_snake_case_ids() {
    let src = "direction LR\nfoo_1[\"F\"]\nbar2[\"B\"]\nfoo_1 --> bar2\n";
    let doc = Parser::parse(src).unwrap();
    assert_eq!(doc.nodes.len(), 2);
}

#[test]
fn rejects_trailing_underscore() {
    let src = "direction LR\nfoo_[\"F\"]\n";
    let err = Parser::parse(src).unwrap_err();
    assert!(err.iter().any(|d| d.code == "E0100"));
}

#[test]
fn rejects_uppercase_in_id() {
    let src = "direction LR\nFoo[\"F\"]\n";
    let err = Parser::parse(src).unwrap_err();
    assert!(err.iter().any(|d| d.code == "E0100"));
}
