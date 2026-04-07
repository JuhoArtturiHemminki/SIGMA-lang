mod lexer;
mod parser;
mod compiler;
mod vortex_error;
mod vortex_core;
mod vortex_runtime;

use std::fs;
use std::path::Path;
use std::io::{self, Write};
use crate::lexer::tokenize;
use crate::parser::Parser;
use crate::compiler::Compiler;
use crate::vortex_runtime::VortexRuntime;

fn main() {
    println!("\x1b[1;36mΣ SIGMA-OS\x1b[0m | Geometric Stabilization 
Kernel");
    println!("{:-<65}", "");

    let build_dir = Path::new("./build_assets");
    let mut runtime = VortexRuntime::new();
    let compiler = Compiler::new("x86_64-sigma-native");

    if let Ok(entries) = fs::read_dir(build_dir) {
        let mut entries_vec: Vec<_> = entries.flatten().collect();
        entries_vec.sort_by_key(|e| e.file_name());

        for entry in entries_vec {
            let path = entry.path();
            let name = 
path.file_name().unwrap().to_string_lossy().to_string();

            if name.ends_with(".sgm") {
                let source = 
fs::read_to_string(&path).unwrap_or_default();
                let tokens = tokenize(&source);
                let parser = Parser::new(tokens);
                
                if let Err(e) = parser.check_geometry() {
                    println!("\x1b[1;31m[ERROR]\x1b[0m In module '{}':", 
name);
                    e.emit();
                } else {
                    let _ = compiler.compile(&source, &path);
                }
            }
        }
    }

    println!("\x1b[1;32mBOOT COMPLETE\x1b[0m | Sigma Interactive Shell 
Active");
    loop {
        print!("\x1b[1;35mσ>\x1b[0m ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" { break; }
        if input == "clear" { print!("{esc}[2J{esc}[1;1H", esc = 27 as 
char); continue; }
        if input == "stabilize" { runtime.stabilize(); continue; }
        if input == "heap" { println!("\x1b[1;32m[DATA]\x1b[0m Records: 
{}", runtime.heap.len()); continue; }

        let tokens = tokenize(input);
        let mut angle_found = None;
        for t in &tokens {
            if let lexer::Token::Angle(a) = t { angle_found = Some(*a); }
        }

        let parser = Parser::new(tokens);
        if let Some(a) = angle_found {
            runtime.request_access(a);
        } else {
            parser.parse_all();
        }
    }
}

