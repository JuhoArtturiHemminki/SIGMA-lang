mod lexer;
mod parser;
mod compiler;
mod vortex_error;
mod vortex_core;
mod vortex_runtime;
mod symmetry_verifier;

use std::io::{self, Write};
use crate::lexer::{tokenize, Token};
use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::vortex_runtime::VortexRuntime;
use crate::symmetry_verifier::SymmetryVerifier;

fn main() {
    println!("\x1b[1;36mΣ SIGMA-OS\x1b[0m | Symmetry-Invariant Engine 
Active");
    println!("{:-<65}", "");
    let mut runtime = VortexRuntime::new();
    let compiler = Compiler::new("x86_64-sigma-native");

    loop {
        print!("\x1b[1;35mσ>\x1b[0m ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = input.trim();
        if cmd == "exit" { break; }
        if cmd == "stabilize" { runtime.stabilize(); continue; }

        if cmd.contains('~') {
            let parts: Vec<&str> = cmd.split('~').collect();
            let original = parts[0].trim();
            if SymmetryVerifier::verify_mirror_integrity(original) {
                println!("\x1b[1;32m  [OK]\x1b[0m Symmetry Stable.");
                runtime.mirror_process(original);
            } else {
                let err = vortex_error::VortexError::PhaseCollision(180, 
original.to_string());
                err.emit();
            }
            continue;
        }

        let tokens = tokenize(cmd);
        let parser = Parser::new(tokens.clone());
        match parser.check_geometry() {
            Ok(_) => {
                parser.parse_all();
                for token in tokens {
                    if let Token::Angle(a) = token { 
runtime.request_access(a); }
                }
                let _ = compiler.compile(cmd, 
std::path::Path::new("session.sgm"));
            },
            Err(e) => e.emit(),
        }
    }
}

