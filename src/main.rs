use crate::aws::{AwsSym, ParseTree};
use crate::lex::{error, read_lex_file};

use std::env;

mod actions;
mod aws;
mod lex;
mod symbols;

use aws::parser::Parser;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let dslf = &args[1];
        println!("=> running file: {}", dslf);
        match read_lex_file(dslf.as_str()) {
            Ok(mut my_scanner) => {
                // println!("{} read!", my_scanner.source);
                // println!("contents: {}", my_scanner.contents);
                my_scanner.scan_tokens();
                if my_scanner.errors.len() > 0 {
                    error(0, "Lexer had errors!");
                }
                let mut aws_parser = Parser::new(my_scanner.tokens.as_ref());
                match aws_parser.parse() {
                    Ok(aws_node) => {
                        println!("\nparse tree:\n{}", aws_node.print_ast(2));
                        match symbols::walk_ast(&aws_node) {
                            Ok(aws_sym) => {
                                println!("Aws resources:{}", aws_sym);
                                match actions::apply_aws(&aws_sym).await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        eprintln!("Error: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("AST -> Symbol Error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("err: {}", e.to_string())
            }
        }
    } else {
        println!("** No application file given! **")
    }
}
