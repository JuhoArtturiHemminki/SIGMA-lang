#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Token {
    Axis, Step, Rotate, Link, Point, Sync,
    Angle(u32), Identifier(String), Number(u32),
    Arrow, At, OpenBrace, CloseBrace, OpenParen, CloseParen, Semicolon,
    Plus, Minus, Star, Slash, Equals, Less, Greater, Dot,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '+' => { tokens.push(Token::Plus); chars.next(); }
            '-' => {
                chars.next();
                if chars.peek() == Some(&'>') { tokens.push(Token::Arrow); 
chars.next(); }
                else { tokens.push(Token::Minus); }
            }
            '*' => { tokens.push(Token::Star); chars.next(); }
            '/' => { tokens.push(Token::Slash); chars.next(); }
            '<' => { tokens.push(Token::Less); chars.next(); }
            '>' => { tokens.push(Token::Greater); chars.next(); }
            '=' => { tokens.push(Token::Equals); chars.next(); }
            '.' => { tokens.push(Token::Dot); chars.next(); }
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
            _ if c.is_digit(10) => {
                let mut s = String::new();
                while let Some(&curr) = chars.peek() {
                    if curr.is_digit(10) { s.push(curr); chars.next(); }
                    else { break; }
                }
                if let Ok(n) = s.parse::<u32>() { 
tokens.push(Token::Number(n)); }
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
                    "link" => tokens.push(Token::Link),
                    "point" => tokens.push(Token::Point),
                    "sync" => tokens.push(Token::Sync),
                    _ => tokens.push(Token::Identifier(s)),
                }
            }
            _ => { chars.next(); }
        }
    }
    tokens
}

