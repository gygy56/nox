mod ast;
mod codegen;
mod diagnostics;
mod lexer;
mod parser;
mod token;
mod types;
mod type_checker;

use lexer::Lexer;
use parser::Parser;
use token::Token;
use type_checker::TypeChecker;

fn main() {
    let source: String = r#"stck a = 10 / 3"#.to_string();


    let mut lexer: Lexer = Lexer::new(source);

    let tokens: Vec<Token> = match lexer.tokenize() {
        Ok(tokens) => tokens,

        Err(error) => {
            eprintln!("Lexer error: {error}");
            return;
        }
    };

    println!("TOKENS:");

    for token in &tokens {
        println!("{token:?}");
    }

    let mut parser: Parser = Parser::new(tokens);

    match parser.parse() {
        Ok(program) => {
            println!("\nAST:");
            let mut checker = TypeChecker::new();

            match checker.check(&program) {
                Ok(()) => println!("Type check: OK"),
                Err(error) => println!("type error: {}", error),
            }
            println!("{program:#?}");
        }

        Err(error) => {
            eprintln!("Parser error: {error}");
        }
    }
}