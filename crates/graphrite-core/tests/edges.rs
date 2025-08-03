use graphrite_core::parser::Parser;

#[test]
fn parses_flow_and_conditional_edges() {
    let src = "direction LR\na[\"A\"]\nb[\"B\"]\na --> b\na -.-> b\n";
    let doc = Parser::parse(src).unwrap();
    assert_eq!(doc.edges.len(), 2);
    assert!(matches!(
        doc.edges[0].kind,
        graphrite_core::ast::EdgeKind::Flow
    ));
    assert!(matches!(
        doc.edges[1].kind,
        graphrite_core::ast::EdgeKind::Conditional
    ));
}

#[test]
fn edge_spans_present() {
    let src = "direction LR\na[\"A\"]\nb[\"B\"]\na --> b\n";
    let doc = Parser::parse(src).unwrap();
    assert!(doc.edges[0].span.is_some());
}
