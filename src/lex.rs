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
    Equal,
    Dot,
    Comma,
    Star,
    // operators
    Less,
    Greater,
    Bang,
    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Minus,
    // End Of File token
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
            &TokenType::Dot => "DOT",
            &TokenType::Star => "STAR",
            &TokenType::Comma => "COMMA",
            &TokenType::Minus => "MINUS",
            &TokenType::Equal => "EQUAL",
            &TokenType::Bang => "BANG",
            &TokenType::Less => "LESS",
            &TokenType::Greater => "GREATER",
            &TokenType::BangEqual => "BANG_EQUAL",
            &TokenType::EqualEqual => "EQUAL_EQUAL",
            &TokenType::LessEqual => "LESS_EQUAL",
            &TokenType::GreaterEqual => "GREATER_EQUAL",
            &TokenType::EoF => "EOF",
        };
        write!(f, "type: {}", tok_s)
    }
}

pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<String>,
    pub(crate) line_no: usize,
}

impl Token {
    pub(crate) fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
        line_no: usize,
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
        let lit = match self.literal.as_ref() {
            Some(x) => x,
            _ => &String::from(""),
        };
        write!(
            f,
            "type {}, lexeme {}, literal {}",
            self.token_type,
            self.lexeme,
            lit.to_string(),
        )
    }
}

// Scanner
pub(crate) struct Scanner {
    pub(crate) source: String,
    pub(crate) contents: String,
    pub(crate) tokens: Vec<Token>,
    pub(crate) errors: Vec<String>,
    pub(crate) start: usize,
    pub(crate) current: usize,
    pub(crate) line: usize,
}

impl Scanner {
    pub(crate) fn scan_tokens(&mut self) {
        // for tok in self.contents.split_ascii_whitespace().into_iter() {
        //     println!("tok: {}", tok);
        // }
        while self.current < self.contents.len() {
            self.start = self.current;
            self.scan_token();
        }
        let eof_tok = Token::new(TokenType::EoF, "".to_string(), None, self.line);
        self.tokens.push(eof_tok);
    }

    //Read the current character and advance the pointer
    pub(crate) fn advance(&mut self) -> Option<char> {
        let c = &self.contents[self.current..self.current + 1];
        self.current += 1;
        return c.chars().next();
    }

    pub(crate) fn add_token(&mut self, tok_type: TokenType) {
        let curr_str = &self.contents[self.start..self.current];
        self.tokens
            .push(Token::new(tok_type, curr_str.to_string(), None, self.line));
    }

    pub(crate) fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            Some('(') => self.add_token(TokenType::LeftParen),
            Some(')') => self.add_token(TokenType::RightParen),
            Some('{') => self.add_token(TokenType::LeftBrace),
            Some('}') => {
                self.add_token(TokenType::RightBrace);
                println!("=> {}", "}")
            }
            Some(';') => self.add_token(TokenType::SemiColon),
            Some(':') => self.add_token(TokenType::Colon),
            Some('.') => self.add_token(TokenType::Dot),
            Some(',') => self.add_token(TokenType::Comma),
            Some('*') => self.add_token(TokenType::Star),
            Some('-') => self.add_token(TokenType::Minus),
            Some('<') => {
                // peek, if '=', then LessEqual
                // otherwise emit Equal
                self.add_token(TokenType::Less)
            }
            Some('>') => {
                // peek, if '=', then GreaterEqual
                // otherwise emit Greater
                self.add_token(TokenType::Greater)
            }
            Some('!') => {
                // peek, if '=', then BangEqual
                // otherwise emit bang
                self.add_token(TokenType::Bang)
            }
            Some('=') => {
                // peek, if '=', then EqualEqual
                // otherwie emit Equal
                self.add_token(TokenType::Equal)
            }
            _ => {
                println!("unrecognised token: {}", c.unwrap_or_default());
            }
        }
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
