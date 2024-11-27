use crate::parser::expr::{Expr, Literal, Visitor};
mod parser;
use parser::Parser;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;
use parser::ast_printer;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut parser = Parser::new(file_contents);
                parser.print_tokens();
                exit(parser.exit_code)
            } else {
                println!("EOF  null")
            }
        }
        "parse_test" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let mut parser = Parser::new(file_contents);
                let mut printer = ast_printer::AstPrinter;
                let temp = Expr::Literal(Box::new(Literal { value: "essa".to_string() }));
                println!("{}", printer.print(temp));
                exit(parser.exit_code)
            } else {
                println!("EOF  null")
            }
        }

        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
