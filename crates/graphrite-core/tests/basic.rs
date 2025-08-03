use graphrite_core::parser::Parser;

#[test]
fn parse_bluesky_nodes() {
    let src = include_str!("../../../samples/valid/bluesky.mmd");
    let doc = Parser::parse(src).expect("parse");
    assert!(doc.nodes.len() >= 5);
}
