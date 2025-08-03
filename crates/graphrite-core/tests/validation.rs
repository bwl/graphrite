use graphrite_core::parser::Parser;

#[test]
fn fails_without_direction_first_line() {
    let src = "a[\"A\"]\n";
    let errs = Parser::parse(src).unwrap_err();
    assert!(errs.iter().any(|e| e.code == "E0001"));
}

#[test]
fn fails_unquoted_label() {
    let src = "direction LR\na[A]\n";
    let errs = Parser::parse(src).unwrap_err();
    assert!(errs.iter().any(|e| e.code == "E0003"));
}

#[test]
fn parses_edges_and_nodes_present() {
    let src = "direction LR\na[\"A\"]\nb[\"B\"]\na --> b\n";
    let doc = Parser::parse(src).expect("parse");
    assert_eq!(doc.nodes.len(), 2);
    assert_eq!(doc.edges.len(), 1);
}
