use graphrite_core::parser::Parser;

#[test]
fn parse_bluesky_nodes() {
    let src = include_str!("../../../samples/valid/bluesky.mmd");
    let doc = Parser::parse(src).unwrap();
    assert!(doc.nodes.len() >= 5);
}

#[test]
fn parse_bluesky_no_orphans() {
    let src = include_str!("../../../samples/valid/bluesky.mmd");
    let doc = Parser::parse(src).unwrap();
    let mut deg = std::collections::BTreeMap::new();
    for n in &doc.nodes {
        deg.insert(&n.id, 0usize);
    }
    for e in &doc.edges {
        *deg.get_mut(&e.from).unwrap() += 1;
        *deg.get_mut(&e.to).unwrap() += 1;
    }
    assert!(deg.values().all(|d| *d > 0));
}
