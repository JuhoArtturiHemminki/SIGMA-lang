#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Axis, Step, Rotate, Link, Point, Sync,
    Angle(u32), Identifier(String), Number(u32),
    ValueClass(u32), Mirror,
    Arrow, At, OpenBrace, CloseBrace, OpenParen, CloseParen, Semicolon,
    Plus, Minus, Star, Slash, Equals, Less, Greater, Dot,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '~' => { tokens.push(Token::Mirror); chars.next(); }
            ':' => { chars.next(); }
            _ if c.is_digit(10) => {
                let mut s = String::new();
                while let Some(&curr) = chars.peek() {
                    if curr.is_digit(10) { s.push(curr); chars.next(); }
                    else { break; }
                }
                if chars.peek() == Some(&':') {
                    if let Ok(n) = s.parse::<u32>() { 
tokens.push(Token::ValueClass(n)); }
                } else {
                    if let Ok(n) = s.parse::<u32>() { 
tokens.push(Token::Number(n)); }
                }
            }
            '@' => {
                tokens.push(Token::At);
                chars.next();
                let mut s = String::new();
                while let Some(&curr) = chars.peek() {
                    if curr.is_digit(10) { s.push(curr); chars.next(); }
                    else { break; }
                }
                if let Ok(n) = s.parse::<u32>() { 
tokens.push(Token::Angle(n)); }
            }
            _ if c.is_alphabetic() => {
                let mut s = String::new();
                while let Some(&curr) = chars.peek() {
                    if curr.is_alphanumeric() || curr == '_' { 
s.push(curr); chars.next(); }
                    else { break; }
                }
                match s.as_str() {
                    "axis" => tokens.push(Token::Axis),
                    "step" => tokens.push(Token::Step),
                    "rotate" => tokens.push(Token::Rotate),
                    _ => tokens.push(Token::Identifier(s)),
                }
            }
            ' ' | '\n' | '\r' | '\t' => { chars.next(); }
            _ => { chars.next(); }
        }
    }
    tokens
}

