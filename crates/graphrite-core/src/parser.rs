use crate::ast::*;
use crate::error::Diagnostic;
use crate::lexer::{Lexer, Token, TokenKind};

pub struct Parser {
    toks: Vec<Token>,
    idx: usize,
}

impl Parser {
    pub fn parse(src: &str) -> Result<Document, Vec<Diagnostic>> {
        let toks = Lexer::tokenize(src);
        let mut p = Parser { toks, idx: 0 };
        p.document()
    }

    fn peek(&self) -> &Token { &self.toks[self.idx] }
    fn bump(&mut self) { if self.idx < self.toks.len() - 1 { self.idx += 1; } }

    fn expect_newline(&mut self) { if matches!(self.peek().kind, TokenKind::Newline) { self.bump(); } }

    fn make_span(tok: &Token, end_tok: &Token) -> Span {
        Span { start: Position { line: tok.line, col: tok.col }, end: Position { line: end_tok.line, col: end_tok.col } }
    }

    fn is_snake_case(id: &str) -> bool {
        if id.is_empty() { return false; }
        let bytes = id.as_bytes();
        if !(bytes[0] as char).is_ascii_lowercase() { return false; }
        if bytes[bytes.len()-1] as char == '_' { return false; }
        for ch in id.chars() {
            if ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' { continue; }
            return false;
        }
        true
    }

    fn document(&mut self) -> Result<Document, Vec<Diagnostic>> {
        let mut diags = Vec::new();
        let mut metadata = None;
        let mut direction = None;
        let mut nodes: Vec<Node> = Vec::new();
        let mut edges: Vec<Edge> = Vec::new();
        let mut line_start_idx = 0usize;
        let mut line_start_col = 1usize;
        while let TokenKind::Comment(text) = &self.peek().kind {
            if text.trim_start().starts_with("Diagram:") {
                let title = text.trim_start().trim_start_matches("Diagram:").trim().to_string();
                metadata = Some(Metadata{ title: Some(title), tags: None });
            }
            self.bump();
            self.expect_newline();
        }
        match (&self.peek().kind, &self.toks.get(self.idx+1).map(|t| &t.kind)) {
            (TokenKind::DirectionKw, Some(TokenKind::Identifier(dir))) => {
                let d = match dir.as_str() { "LR" => Direction::LR, "TD" => Direction::TD, _ => { diags.push(Diagnostic{ code:"E0001".into(), message:"Invalid direction".into(), span: Some(Span{ start: Position{ line: self.peek().line, col: self.peek().col }, end: Position{ line: self.peek().line, col: self.peek().col+1 } })}); Direction::LR } };
                direction = Some(d);
                self.bump(); self.bump();
                self.expect_newline();
            }
            _ => {
                diags.push(Diagnostic{ code:"E0001".into(), message:"Missing direction on first line".into(), span: Some(Span{ start: Position{ line: self.peek().line, col: self.peek().col }, end: Position{ line: self.peek().line, col: self.peek().col } })});
            }
        }
        while !matches!(self.peek().kind, TokenKind::Eof) {
            if matches!(self.peek().kind, TokenKind::Newline) {
                if self.idx > line_start_idx {
                    let start_tok = &self.toks[line_start_idx];
                    let end_tok = &self.toks[self.idx-1];
                    let len = end_tok.col + 0 - line_start_col;
                    if len > 100 {
                        diags.push(Diagnostic{ code: "E0300".into(), message: format!("Line {} exceeds max length ({} > 100)", start_tok.line, len), span: Some(Span{ start: Position{ line: start_tok.line, col: line_start_col }, end: Position{ line: end_tok.line, col: end_tok.col } })});
                    }
                }
                self.bump();
                line_start_idx = self.idx;
                line_start_col = 1;
                continue;
            }
            if matches!(self.peek().kind, TokenKind::Comment(_)) { self.bump(); self.expect_newline(); continue; }

            if self.toks.get(line_start_idx).map(|t| t.line).unwrap_or(0) != self.peek().line { line_start_idx = self.idx; line_start_col = self.peek().col; }

            match &self.peek().kind {
                TokenKind::Identifier(id) => {
                    let nid = id.clone();
                    if matches!(self.toks.get(self.idx+1).map(|t| &t.kind), Some(TokenKind::LBracket)) {
                        let start_tok = self.peek().clone();
                        self.bump();
                        self.bump();
                        let label = match &self.peek().kind { TokenKind::StringLit(s) => { let x=s.clone(); self.bump(); x }, _ => { diags.push(Diagnostic{ code:"E0003".into(), message:"Node label must be quoted".into(), span: Some(Span{ start: Position{ line: self.peek().line, col: self.peek().col }, end: Position{ line: self.peek().line, col: self.peek().col } })}); String::new() } };
                        match self.peek().kind { TokenKind::RBracket => { self.bump(); }, _ => { diags.push(Diagnostic{ code:"E0010".into(), message:"Expected ] after label".into(), span: Some(Span{ start: Position{ line: self.peek().line, col: self.peek().col }, end: Position{ line: self.peek().line, col: self.peek().col } })}); } }
                        if !Self::is_snake_case(&nid) {
                            diags.push(Diagnostic{ code:"E0100".into(), message:"Identifier must be snake_case".into(), span: Some(Self::make_span(&start_tok, &start_tok)) });
                        }
                        nodes.push(Node{ id: nid, label, span: Some(Self::make_span(&start_tok, &self.toks[self.idx-1])) });
                    } else {
                        if let Some(arrow_tok) = self.toks.get(self.idx+1) {
                            let kind = match arrow_tok.kind { TokenKind::ArrowFlow => Some(EdgeKind::Flow), TokenKind::ArrowCond => Some(EdgeKind::Conditional), _ => None };
                            if let Some(k) = kind {
                                if let Some(Token{kind: TokenKind::Identifier(dst_id), ..}) = self.toks.get(self.idx+2) {
                                    let from = nid;
                                    let to = dst_id.clone();
                                    let start_tok = self.peek().clone();
                                    let end_tok = self.toks[self.idx+2].clone();
                                    self.bump();
                                    self.bump();
                                    self.bump();
                                    edges.push(Edge{ from, to, kind: k, span: Some(Self::make_span(&start_tok, &end_tok)) });
                                    while !matches!(self.peek().kind, TokenKind::Newline | TokenKind::Eof) { self.bump(); }
                                } else {
                                    while !matches!(self.peek().kind, TokenKind::Newline | TokenKind::Eof) { self.bump(); }
                                }
                            } else {
                                while !matches!(self.peek().kind, TokenKind::Newline | TokenKind::Eof) { self.bump(); }
                            }
                        } else {
                            while !matches!(self.peek().kind, TokenKind::Newline | TokenKind::Eof) { self.bump(); }
                        }
                    }
                }
                TokenKind::Newline => { self.bump(); }
                TokenKind::Comment(_) => { self.bump(); }
                TokenKind::ArrowFlow | TokenKind::ArrowCond => {
                    while !matches!(self.peek().kind, TokenKind::Newline | TokenKind::Eof) { self.bump(); }
                }
                _ => {
                    if let TokenKind::Identifier(src_id) = &self.peek().kind {
                        if let Some(arrow_tok) = self.toks.get(self.idx+1) {
                            let kind = match arrow_tok.kind { TokenKind::ArrowFlow => Some(EdgeKind::Flow), TokenKind::ArrowCond => Some(EdgeKind::Conditional), _ => None };
                            if let Some(k) = kind {
                                if let Some(Token{kind: TokenKind::Identifier(dst_id), ..}) = self.toks.get(self.idx+2) {
                                    let from = src_id.clone();
                                    let to = dst_id.clone();
                                    let start_tok = self.peek().clone();
                                    let end_tok = self.toks[self.idx+2].clone();
                                    self.bump();
                                    self.bump();
                                    self.bump();
                                    edges.push(Edge{ from, to, kind: k, span: Some(Self::make_span(&start_tok, &end_tok)) });
                                    while !matches!(self.peek().kind, TokenKind::Newline | TokenKind::Eof) { self.bump(); }
                                    continue;
                                }
                            }
                        }
                    }
                    self.bump();
                }
            }
        }
        if !diags.is_empty() { return Err(diags); }
        let doc = Document{ version: "1".into(), directives: Directives{ direction: direction.unwrap_or(Direction::LR) }, metadata, nodes, edges };
        let mut errors = Vec::new();
        let mut ids = std::collections::BTreeSet::new();
        for n in &doc.nodes { ids.insert(n.id.clone()); }
        for e in &doc.edges {
            if !ids.contains(&e.from) {
                errors.push(Diagnostic{ code: "E0201".into(), message: format!("Edge from references unknown node '{}'", e.from), span: e.span.clone() });
            }
            if !ids.contains(&e.to) {
                errors.push(Diagnostic{ code: "E0202".into(), message: format!("Edge to references unknown node '{}'", e.to), span: e.span.clone() });
            }
        }
        if !errors.is_empty() { return Err(errors); }
        let mut degree: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
        for n in &doc.nodes { degree.insert(&n.id, 0); }
        for e in &doc.edges {
            if let Some(d) = degree.get_mut(e.from.as_str()) { *d += 1; }
            if let Some(d) = degree.get_mut(e.to.as_str()) { *d += 1; }
        }
        let mut orphan_errors = Vec::new();
        for n in &doc.nodes {
            let deg = degree.get(n.id.as_str()).copied().unwrap_or(0);
            if deg == 0 {
                orphan_errors.push(Diagnostic{ code: "E0203".into(), message: format!("Orphan node '{}' has no edges", n.id), span: n.span.clone() });
            }
        }
        if !orphan_errors.is_empty() { return Err(orphan_errors); }
        Ok(doc)
    }
}
