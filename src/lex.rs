use std::error;
use std::fmt;
use std::fs;
use std::io::Error;

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
    Div,
    // others
    Comment,
    StringLiteral,
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
            &TokenType::Div => "DIV",
            &TokenType::Minus => "MINUS",
            &TokenType::Equal => "EQUAL",
            &TokenType::Bang => "BANG",
            &TokenType::Less => "LESS",
            &TokenType::Greater => "GREATER",
            &TokenType::BangEqual => "BANG_EQUAL",
            &TokenType::EqualEqual => "EQUAL_EQUAL",
            &TokenType::LessEqual => "LESS_EQUAL",
            &TokenType::GreaterEqual => "GREATER_EQUAL",
            &TokenType::Comment => "COMMENT",
            &TokenType::StringLiteral => "STRING_LITERAL",
            &TokenType::EoF => "EOF",
        };
        write!(f, "tok: {}", tok_s)
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
            "[{}] type {}, lexeme {}, literal {}",
            self.line_no,
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
    // peek into next character but don't consume
    pub(crate) fn peek(&self) -> Option<char> {
        if self.current < self.contents.len() {
            let c = &self.contents[self.current..self.current + 1];
            return c.chars().next();
        }
        return None;
    }

    //Read the current character and advance the pointer
    pub(crate) fn advance(&mut self) -> Option<char> {
        let c = &self.contents[self.current..self.current + 1];
        self.current += 1;
        return c.chars().next();
    }

    // Read until end of line, for handling comments
    pub(crate) fn read_until_eol(&mut self) {
        while self.current < self.contents.len() {
            let c = self.advance();
            match c {
                Some('\n') => {
                    self.line += 1;
                    return;
                }
                Some('\r') => {
                    return;
                }
                _ => {}
            }
        }
    }

    pub(crate) fn read_until_eo_quote(&mut self) {
        while self.current < self.contents.len() {
            let c = self.advance();
            match c {
                Some('\n') => {
                    self.line += 1;
                }
                Some('\r') => {
                    self.line += 1;
                }
                Some('"') => return,
                _ => {}
            }
        }
    }

    pub(crate) fn add_token(&mut self, tok_type: TokenType) {
        let curr_str = &self.contents[self.start..self.current];
        self.tokens
            .push(Token::new(tok_type, curr_str.to_string(), None, self.line));
    }

    pub(crate) fn scan_tokens(&mut self) {
        // for tok in self.contents.split_ascii_whitespace().into_iter() {
        //     println!("tok: {}", tok);
        // }
        while self.current < self.contents.len() {
            println!("line: {}", self.line);
            self.start = self.current;
            self.scan_token();
        }
        let eof_tok = Token::new(TokenType::EoF, "".to_string(), None, self.line);
        self.tokens.push(eof_tok);
    }

    pub(crate) fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // handle single character ones
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
            // handle two character operators
            Some('<') => {
                // peek, if '=', then LessEqual
                // otherwise emit Equal
                match self.peek() {
                    Some('=') => {
                        self.add_token(TokenType::LessEqual);
                        self.current += 1;
                        println!("=> {}", "<=")
                    }
                    _ => self.add_token(TokenType::Less),
                }
            }
            Some('>') => {
                // peek, if '=', then GreaterEqual
                // otherwise emit Greater
                match self.peek() {
                    Some('=') => {
                        self.add_token(TokenType::GreaterEqual);
                        self.current += 1;
                        println!("=> {}", ">=")
                    }
                    _ => self.add_token(TokenType::Greater),
                }
            }
            Some('!') => {
                // peek, if '=', then BangEqual
                // otherwise emit bang
                match self.peek() {
                    Some('=') => {
                        self.add_token(TokenType::BangEqual);
                        self.current += 1;
                        println!("=> {}", "!=")
                    }
                    _ => self.add_token(TokenType::Bang),
                }
            }
            Some('=') => {
                // peek, if '=', then EqualEqual
                // otherwie emit Equal
                match self.peek() {
                    Some('=') => {
                        self.add_token(TokenType::EqualEqual);
                        self.current += 1;
                        println!("=> {}", "==")
                    }
                    _ => self.add_token(TokenType::Equal),
                }
            }
            Some('/') => {
                // peek, if '/', then advance until \n, emit Comment
                // otherwie emit Div
                match self.peek() {
                    Some('/') => {
                        println!("=> {}", "//");
                        self.read_until_eol();
                        self.add_token(TokenType::Comment);
                        self.current += 1;
                    }
                    _ => self.add_token(TokenType::Div),
                }
            }
            // handle string literal
            Some('"') => {
                // start after the quote
                self.start += 1;
                self.read_until_eo_quote();
                // since at current end quote ", move one step back
                self.current -= 1;
                println!(
                    "=> in a string: {}",
                    &self.contents[self.start..self.current]
                );
                self.add_token(TokenType::StringLiteral);
                // now we step forward by 2 to lose end quote "
                self.current += 2;
            }
            // handle whitespace and CRLF
            Some(' ') => {}
            Some('\t') => {}
            Some('\r') => {}
            Some('\n') => self.line += 1,
            _ => {
                println!("unrecognised token: *{}*", c.unwrap_or_default());
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
        line: 1,
    };
    Ok(my_scanner)
}
