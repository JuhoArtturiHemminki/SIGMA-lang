use crate::lexer::Token;
use crate::vortex_error::VortexError;

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    pub fn check_geometry(&self) -> Result<(), VortexError> {
        let mut occupied_angles = Vec::new();
        for token in &self.tokens {
            match token {
                Token::Angle(a) => {
                    if occupied_angles.contains(a) {
                        return Err(VortexError::PhaseCollision(*a, 
"Sector_Overlap".to_string()));
                    }
                    occupied_angles.push(*a);
                }
                Token::Sync | Token::Rotate => occupied_angles.clear(),
                _ => {}
            }
        }
        Ok(())
    }

    pub fn parse_all(&self) {
        for token in &self.tokens {
            match token {
                Token::Angle(a) => println!("\x1b[1;34m[PARSER]\x1b[0m 
Phase Gate: {}°", a),
                Token::Identifier(id) => 
println!("\x1b[1;30m[AST-NODE]\x1b[0m Sym: {}", id),
                Token::Plus | Token::Star => 
println!("\x1b[1;32m[AST-LINK]\x1b[0m Math link."),
                Token::Number(n) => println!("\x1b[1;30m[AST-LEAF]\x1b[0m 
Const: {}", n),
                _ => {}
            }
        }
    }
}

