use graphrite_core::parser::Parser;

#[test]
fn missing_direction_has_span() {
    let src = "a[\"A\"]\n";
    let errs = Parser::parse(src).unwrap_err();
    let e = errs.iter().find(|e| e.code == "E0001").unwrap();
    assert!(e.span.is_some());
}

#[test]
fn node_label_must_be_quoted_error() {
    let src = "direction LR\na[unquoted]\n";
    let errs = Parser::parse(src).unwrap_err();
    assert!(errs.iter().any(|e| e.code == "E0003"));
}
