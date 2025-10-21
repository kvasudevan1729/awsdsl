use crate::lex::{error, read_lex_file};
use std::env;

mod lex;

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
            }
            Err(e) => {
                println!("err: {}", e.to_string())
            }
        }
    } else {
        println!("** no file given! **")
    }
}
