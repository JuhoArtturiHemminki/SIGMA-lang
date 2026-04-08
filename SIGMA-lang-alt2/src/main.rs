mod lexer;
mod parser;
mod compiler;
mod vortex_error;
mod vortex_core;
mod vortex_runtime;

use std::io::{self, Write};
use crate::lexer::{tokenize, Token};
use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::vortex_runtime::VortexRuntime;

fn main() {
    println!("\x1b[1;36mΣ SIGMA-OS\x1b[0m | Geometric Engine Active");
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
        if cmd == "heap" { println!("\x1b[1;32m[DATA]\x1b[0m Heap: {:?}", 
runtime.heap); continue; }

        let tokens = tokenize(cmd);
        let parser = Parser::new(tokens.clone());

        if let Err(e) = parser.check_geometry() {
            e.emit();
            continue;
        }

        parser.parse_all();

        if cmd.contains('~') {
            runtime.mirror_process(cmd);
        }

        for token in &tokens {
            match token {
                Token::Angle(a) => runtime.request_access(*a),
                Token::Identifier(id) => { runtime.heap.insert(id.clone(), 
1.0); },
                _ => {}
            }
        }

        let _ = compiler.compile(cmd, 
std::path::Path::new("session.sgm"));
    }
}

