use graphrite_core::parser::Parser;
use graphrite_core::ast::EdgeKind;

#[test]
fn parses_edges_flow_and_conditional() {
    let src = r#"direction LR

a["A"]
b["B"]
a --> b
b -.-> a
"#;
    let doc = Parser::parse(src).expect("parse");
    assert_eq!(doc.edges.len(), 2);
    assert_eq!(doc.edges[0].from, "a");
    assert_eq!(doc.edges[0].to, "b");
    assert!(matches!(doc.edges[0].kind, EdgeKind::Flow));
    assert!(matches!(doc.edges[1].kind, EdgeKind::Conditional));
}
