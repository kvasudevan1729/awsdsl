use crate::aws::ParseTree;
use crate::lex::{error, read_lex_file};

use std::env;

mod aws;
mod lex;
mod symbols;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let lexf = &args[1];
        println!("lex file: {}", lexf);
        match read_lex_file(lexf.as_str()) {
            Ok(mut my_scanner) => {
                println!("{} read!", my_scanner.source);
                println!("contents: {}", my_scanner.contents);
                my_scanner.scan_tokens();
                if my_scanner.errors.len() > 0 {
                    error(0, "HAD ERROR");
                }
                symbols::build_parse_tree(&my_scanner);
                let mut aws_parser = aws::parser::Parser::new(my_scanner.tokens.as_ref());
                match aws_parser.parse() {
                    Ok(aws_node) => {
                        println!("\nparse tree:\n{}", aws_node.ast(2))
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
        println!("** no file given! **")
    }
}
