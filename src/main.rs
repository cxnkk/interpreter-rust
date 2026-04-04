#![allow(unused_variables)]
mod scanner;

use std::env;
use std::fs;

struct Scanner {
    source: String,
}
enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semicolon,
    Slash,
    Eof,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

enum Literal {
    None,
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

// TODO: Add some helper functions, enums and structs.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            let mut had_error = false;

            for c in file_contents.chars() {
                match c {
                    '(' => println!("LEFT_PAREN ( null"),
                    ')' => println!("RIGHT_PAREN ) null"),
                    '{' => println!("LEFT_BRACE {{ null"),
                    '}' => println!("RIGHT_BRACE }} null"),
                    '*' => println!("STAR * null"),
                    '.' => println!("DOT . null"),
                    ',' => println!("COMMA , null"),
                    '+' => println!("PLUS + null"),
                    '-' => println!("MINUS - null"),
                    ';' => println!("SEMICOLON ; null"),
                    _ => {
                        eprintln!("[line 1] Error: Unexpected character: {}", c);
                        had_error = true;
                    }
                }
            }
            println!("EOF  null");

            if had_error {
                std::process::exit(65);
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
