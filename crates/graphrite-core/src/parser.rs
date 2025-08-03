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

    fn document(&mut self) -> Result<Document, Vec<Diagnostic>> {
        let mut diags = Vec::new();
        let mut metadata = None;
        let mut direction = None;
        let mut nodes = Vec::new();
        let mut edges: Vec<Edge> = Vec::new();
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
                let d = match dir.as_str() { "LR" => Direction::LR, "TD" => Direction::TD, _ => { diags.push(Diagnostic{ code:"E0001".into(), message:"Invalid direction".into(), span: None}); Direction::LR } };
                direction = Some(d);
                self.bump(); self.bump();
                self.expect_newline();
            }
            _ => {
                diags.push(Diagnostic{ code:"E0001".into(), message:"Missing direction on first line".into(), span: None});
            }
        }
        while !matches!(self.peek().kind, TokenKind::Eof) {
            // skip empty lines
            if matches!(self.peek().kind, TokenKind::Newline) { self.bump(); continue; }
            // skip comments
            if matches!(self.peek().kind, TokenKind::Comment(_)) { self.bump(); self.expect_newline(); continue; }

            match &self.peek().kind {
                // node declaration: id["Label"]
                // edge: id --> id (we'll skip edges for now)

                TokenKind::Identifier(id) => {
                    // Distinguish node decl vs edge: node decl if next is LBracket; otherwise treat as start of edge and consume line
                    let nid = id.clone();
                    // lookahead without consuming
                    if matches!(self.toks.get(self.idx+1).map(|t| &t.kind), Some(TokenKind::LBracket)) {
                        self.bump(); // id
                        self.bump(); // [
                        let label = match &self.peek().kind { TokenKind::StringLit(s) => { let x=s.clone(); self.bump(); x }, _ => { diags.push(Diagnostic{ code:"E0003".into(), message:"Node label must be quoted".into(), span: None}); String::new() } };
                        match self.peek().kind { TokenKind::RBracket => { self.bump(); }, _ => { diags.push(Diagnostic{ code:"E0010".into(), message:"Expected ] after label".into(), span: None}); } }
                        nodes.push(Node{ id: nid, label, span: None });
                    } else {
                        // try edge parse here: id (at idx) followed by arrow and id
                        if let Some(arrow_tok) = self.toks.get(self.idx+1) {
                            let kind = match arrow_tok.kind { TokenKind::ArrowFlow => Some(EdgeKind::Flow), TokenKind::ArrowCond => Some(EdgeKind::Conditional), _ => None };
                            if let Some(k) = kind {
                                if let Some(Token{kind: TokenKind::Identifier(dst_id), ..}) = self.toks.get(self.idx+2) {
                                    let from = nid;
                                    let to = dst_id.clone();
                                    self.bump(); // src id
                                    self.bump(); // arrow
                                    self.bump(); // dst id
                                    edges.push(Edge{ from, to, kind: k, span: None });
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
                    // Unexpected arrow without preceding identifier; consume line
                    while !matches!(self.peek().kind, TokenKind::Newline | TokenKind::Eof) { self.bump(); }
                }
                _ => {
                    // Try to parse edge pattern: id --> id or id -.-> id
                    if let TokenKind::Identifier(src_id) = &self.peek().kind {
                        if let Some(arrow_tok) = self.toks.get(self.idx+1) {
                            let kind = match arrow_tok.kind { TokenKind::ArrowFlow => Some(EdgeKind::Flow), TokenKind::ArrowCond => Some(EdgeKind::Conditional), _ => None };
                            if let Some(k) = kind {
                                if let Some(Token{kind: TokenKind::Identifier(dst_id), ..}) = self.toks.get(self.idx+2) {
                                    let from = src_id.clone();
                                    let to = dst_id.clone();
                                    self.bump(); // src
                                    self.bump(); // arrow
                                    self.bump(); // dst
                                    edges.push(Edge{ from, to, kind: k, span: None });
                                    // consume rest of line
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
        Ok(Document{ version: "1".into(), directives: Directives{ direction: direction.unwrap_or(Direction::LR) }, metadata, nodes, edges })
    }
}
