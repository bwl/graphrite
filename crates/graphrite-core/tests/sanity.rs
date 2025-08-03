use graphrite_core::parser::Parser;

#[test]
fn parses_direction_and_nodes() {
    let src = r#"%% Diagram: Simple

direction LR

a["A"]
b["B"]
a --> b
"#;
    let doc = Parser::parse(src).expect("parse");
    assert_eq!(
        doc.directives.direction as u8,
        graphrite_core::ast::Direction::LR as u8
    );
    assert_eq!(doc.nodes.len(), 2);
    assert_eq!(doc.nodes[0].id, "a");
    assert_eq!(doc.nodes[1].label, "B");
}

#[test]
fn errors_without_direction() {
    let src = "a[\"A\"]\n";
    let errs = Parser::parse(src).unwrap_err();
    assert!(errs.iter().any(|e| e.code == "E0001"));
}

#[test]
fn edges_validate_presence() {
    let src = r#"direction LR

a["A"]
a --> b
b["B"]
"#;
    let doc = Parser::parse(src).expect("parse");
    assert_eq!(doc.nodes.len(), 2);
    assert_eq!(doc.edges.len(), 1);
}
