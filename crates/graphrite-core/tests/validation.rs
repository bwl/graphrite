use graphrite_core::parser::Parser;

#[test]
fn invalid_id_not_snake_case() {
    let src = "direction LR\nFoo[\"Bad\"]\n";
    let err = Parser::parse(src).unwrap_err();
    assert!(err.iter().any(|d| d.code == "E0100"));
}

#[test]
fn dangling_edge_from() {
    let src = "direction LR\nfoo[\"Foo\"]\nbar --> baz\n";
    let err = Parser::parse(src).unwrap_err();
    assert!(err.iter().any(|d| d.code == "E0201" || d.code == "E0202"));
}

#[test]
fn spans_present_on_nodes_and_edges() {
    let src = "direction LR\nfoo[\"Foo\"]\nfoo --> foo\n";
    let doc = Parser::parse(src).unwrap();
    assert!(doc.nodes[0].span.is_some());
    assert!(doc.edges[0].span.is_some());
}

#[test]
fn orphan_nodes_error() {
    let src = "direction LR\nfoo[\"Foo\"]\nbar[\"Bar\"]\nfoo --> foo\n";
    let err = Parser::parse(src).unwrap_err();
    assert!(err.iter().any(|d| d.code == "E0203"));
}

#[test]
fn line_length_limit() {
    let long_label = "A".repeat(120);
    let src = format!("direction LR\na[\"{}\"]\n", long_label);
    let err = Parser::parse(&src).unwrap_err();
    assert!(err.iter().any(|d| d.code == "E0300"));
}
