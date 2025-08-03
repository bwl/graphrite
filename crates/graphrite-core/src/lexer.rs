#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    DirectionKw,
    Identifier(String),
    StringLit(String),
    ArrowFlow,     // -->
    ArrowCond,     // -.- > (represented as -.->)
    LBracket,      // [
    RBracket,      // ]
    Pipe,          // |
    Comment(String),
    Newline,
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token { pub kind: TokenKind, pub line: usize, pub col: usize }

pub struct Lexer;

impl Lexer {
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut toks = Vec::new();
        let mut line_no = 1usize;
        for line in input.split_inclusive('\n') {
            let mut i = 0usize;
            let bytes = line.as_bytes();
            let len = bytes.len();
            while i < len {
                let col = i + 1;
                let c = bytes[i] as char;
                match c {
                    ' ' | '\t' => { i += 1; }
                    '\n' => { toks.push(Token{kind: TokenKind::Newline, line: line_no, col}); i += 1; }
                    '%' if i + 1 < len && bytes[i+1] as char == '%' => {
                        let text = &line[i+2..];
                        let text = text.trim_end_matches('\n').to_string();
                        toks.push(Token{kind: TokenKind::Comment(text), line: line_no, col});
                        // consume rest of line including optional trailing \n via outer split
                        i = len;
                    }
                    '[' => { toks.push(Token{kind: TokenKind::LBracket, line: line_no, col}); i += 1; }
                    ']' => { toks.push(Token{kind: TokenKind::RBracket, line: line_no, col}); i += 1; }
                    '|' => { toks.push(Token{kind: TokenKind::Pipe, line: line_no, col}); i += 1; }
                    '"' => {
                        // string literal until next unescaped quote or EOL
                        let mut j = i + 1;
                        let mut s = String::new();
                        while j < len {
                            let ch = bytes[j] as char;
                            if ch == '"' { break; }
                            if ch == '\\' && j + 1 < len {
                                let nxt = bytes[j+1] as char;
                                match nxt { '"' => { s.push('"'); j += 2; continue; }, '\\' => { s.push('\\'); j += 2; continue; }, _ => {} }
                            }
                            s.push(ch);
                            j += 1;
                        }
                        toks.push(Token{kind: TokenKind::StringLit(s), line: line_no, col});
                        i = if j < len { j + 1 } else { len };
                    }
                    '-' => {
                        // arrows
                        if i + 2 < len && bytes[i+1] as char == '-' && bytes[i+2] as char == '>' {
                            toks.push(Token{kind: TokenKind::ArrowFlow, line: line_no, col});
                            i += 3;
                        } else if i + 3 < len && bytes[i+1] as char == '.' && bytes[i+2] as char == '-' && bytes[i+3] as char == '>' {
                            toks.push(Token{kind: TokenKind::ArrowCond, line: line_no, col});
                            i += 4;
                        } else { i += 1; }
                    }
                    _ => {
                        // identifier/keywords
                        if c.is_ascii_alphabetic() {
                            let mut j = i + 1;
                            while j < len {
                                let ch = bytes[j] as char;
                                if ch.is_ascii_alphanumeric() || ch == '_' { j += 1; } else { break; }
                            }
                            let word = &line[i..j];
                            let word_str = word.trim_end_matches('\n');
                            if word_str == "direction" { toks.push(Token{kind: TokenKind::DirectionKw, line: line_no, col}); }
                            else { toks.push(Token{kind: TokenKind::Identifier(word_str.to_string()), line: line_no, col}); }
                            i = j;
                        } else { i += 1; }
                    }
                }
            }
            if !line.ends_with('\n') { toks.push(Token{kind: TokenKind::Newline, line: line_no, col: 1}); }
            line_no += 1;
        }
        toks.push(Token{kind: TokenKind::Eof, line: line_no, col: 1});
        toks
    }
}
