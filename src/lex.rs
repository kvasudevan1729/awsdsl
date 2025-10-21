use std::error;
use std::fmt;
use std::fs;

// Token stuff
pub(crate) enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,
    Colon,
    EoF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tok_s = match self {
            &TokenType::LeftParen => "LEFT_PAREN",
            &TokenType::RightParen => "RIGHT_PAREN",
            &TokenType::LeftBrace => "LEFT_BRACE",
            &TokenType::RightBrace => "RIGHT_BRACE",
            &TokenType::SemiColon => "SEMICOLON",
            &TokenType::Colon => "COLON",
            &TokenType::EoF => "EOF",
        };
        write!(f, "type: {}", tok_s)
    }
}

pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: String,
    pub(crate) line_no: u32,
}

impl Token {
    pub(crate) fn new(
        token_type: TokenType,
        lexeme: String,
        literal: String,
        line_no: u32,
    ) -> Self {
        Token {
            token_type: token_type,
            lexeme: lexeme,
            literal: literal,
            line_no: line_no,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "type {}, lexeme {}, literal {}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

// Scanner
pub(crate) struct Scanner {
    pub(crate) source: String,
    pub(crate) contents: String,
    pub(crate) tokens: Vec<Token>,
    pub(crate) errors: Vec<String>,
    pub(crate) start: u32,
    pub(crate) current: u32,
    pub(crate) line: u32,
}

impl Scanner {
    pub(crate) fn scan_tokens(&mut self) {
        // for tok in self.contents.split_ascii_whitespace().into_iter() {
        //     println!("tok: {}", tok);
        // }
        // self.errors.push("xxx".to_string());
        while (self.start < self.contents.len()) {
            self.start = self.current;
            self.scan_token();
        }
        let eof_tok = Token::new(TokenType::EoF, "".to_string(), "".to_string(), self.line);
        self.tokens.push(eof_tok);
    }

    pub(crate) fn scan_token(&mut self) {
        //
    }
}

pub(crate) fn error(line: u32, msg: &str) {
    // also add to errors
    report(line, "".to_string().as_str(), &msg);
}

pub(crate) fn report(line: u32, col_loc: &str, msg: &str) {
    println!("[line [{}] Error where: {}: {}] ", line, col_loc, msg);
}

pub(crate) fn read_lex_file(s: &str) -> Result<Scanner, Box<dyn error::Error>> {
    let lexf_s = fs::read_to_string(s)?;
    let my_scanner: Scanner = Scanner {
        source: s.to_string(),
        contents: lexf_s.to_string(),
        tokens: vec![],
        errors: vec![],
        start: 0,
        current: 0,
        line: 0,
    };
    Ok(my_scanner)
}
