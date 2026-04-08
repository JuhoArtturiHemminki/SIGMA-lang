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
    println!("\x1b[1;36mΣ SIGMA-OS\x1b[0m | Mirror-Invariant Computing 
Active");
    println!("{:-<65}", "");

    let mut runtime = VortexRuntime::new();
    let compiler = Compiler::new("x86_64-sigma-native");

    loop {
        print!("\x1b[1;35mσ>\x1b[0m ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_trim = input.trim();

        if input_trim == "exit" { break; }
        if input_trim == "heap" { println!("\x1b[1;32m[DATA]\x1b[0m Heap 
size: {}", runtime.heap.len()); continue; }

        let tokens = tokenize(input_trim);
        let parser = Parser::new(tokens.clone());

        match parser.check_geometry() {
            Ok(_) => {
                let mut current_id = String::new();
                let mut mirror_engaged = false;

                for token in &tokens {
                    match token {
                        Token::ValueClass(c) => runtime.set_class(*c),
                        Token::Angle(a) => runtime.request_access(*a),
                        Token::Mirror => {
                            mirror_engaged = true;
                            println!("\x1b[1;33m  [MIRROR]\x1b[0m Channel 
inversion engaged.");
                        },
                        Token::Identifier(id) => {
                            if mirror_engaged && !current_id.is_empty() {
                                if runtime.mirror_check(&current_id, id) {
                                    println!("\x1b[1;32m  [VALID]\x1b[0m 
Mirror match: {} <=> {}", current_id, id);
                                    runtime.heap.insert(id.clone(), 1.0);
                                } else {
                                    println!("\x1b[1;31m  [ERROR]\x1b[0m 
Symmetry violation!");
                                }
                            }
                            current_id = id.clone();
                            println!("\x1b[1;30m  [DATA]\x1b[0m 
Processing: {}", id);
                        },
                        _ => {}
                    }
                }
                parser.parse_all();
                let _ = compiler.compile(input_trim, 
std::path::Path::new("shell_input.sgm"));
            },
            Err(e) => e.emit(),
        }
    }
}

