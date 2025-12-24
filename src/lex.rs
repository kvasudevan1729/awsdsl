use std::error;
use std::fmt;
use std::fs;

// Token stuff
#[derive(PartialEq)]
pub(crate) enum TokenType {
    // grouping
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    // markers
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
    Number,
    Identifier,
    Keyword,
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
            &TokenType::Number => "NUMBER",
            &TokenType::Identifier => "IDENTIFIER",
            &TokenType::Keyword => "KEYWORD",
            &TokenType::EoF => "EOF",
        };
        write!(f, "tok: {}", tok_s)
    }
}

static KEYWORDS: [&str; 13] = [
    "aws",
    "ec2",
    "ec2_id",
    "name",
    "description",
    "count",
    "app_version",
    "image",
    "ami",
    "subnet_id",
    "instance_type",
    "sg_id",
    "region",
];

pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<String>,
    pub(crate) line_no: usize,
    pub(crate) column_no: usize,
}

impl Token {
    pub(crate) fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
        line_no: usize,
        column_no: usize,
    ) -> Self {
        Token {
            token_type: token_type,
            lexeme: lexeme,
            literal: literal,
            line_no: line_no,
            column_no: column_no,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!(
            "[{}:{}] type: {}, lexeme: {}",
            self.line_no, self.column_no, self.token_type, self.lexeme,
        );
        let lit = match self.literal.as_ref() {
            Some(x) => {
                format!("{}, literal: {}", s, x)
            }
            _ => {
                format!("{}", s)
            }
        };
        write!(f, "{}", lit)
    }
}

/// Scanner
/// source: source file path
/// contents: contents from `source` to parse
/// current: maintains the current pointer in the `contents`
/// line: the current line in `contents`
/// column_no: current column being read in `line`
#[derive(Default)]
pub(crate) struct Scanner {
    pub(crate) source: String,
    pub(crate) contents: String,
    pub(crate) tokens: Vec<Token>,
    pub(crate) errors: Vec<String>,
    pub(crate) start: usize,
    pub(crate) current: usize,
    pub(crate) line: usize,
    pub(crate) column_no: usize,
}

impl Scanner {
    /// Create a new scanner based on the src file path and its contents
    pub(crate) fn new(src_file: String, contents: String) -> Self {
        Scanner {
            source: src_file,
            contents: contents,
            line: 1,
            ..Default::default()
        }
    }

    /// peek into next character but don't consume
    pub(crate) fn peek(&self) -> Option<char> {
        if self.current < self.contents.len() {
            let c = &self.contents[self.current..self.current + 1];
            return c.chars().next();
        }
        return None;
    }

    /// Read the current character and advance the pointer
    pub(crate) fn advance(&mut self) -> Option<char> {
        let c = &self.contents[self.current..self.current + 1];
        self.current += 1;
        self.column_no += 1;
        return c.chars().next();
    }

    /// Read until end of line, for handling comments
    pub(crate) fn read_until_eol(&mut self) {
        while self.current < self.contents.len() {
            let c = self.advance();
            match c {
                Some('\n') => {
                    self.line += 1;
                    self.column_no = 0;
                    return;
                }
                Some('\r') => {
                    self.line += 1;
                    self.column_no = 0;
                    return;
                }
                _ => {}
            }
        }
    }

    /// read until end of a quote, handles multi lines
    pub(crate) fn read_until_eo_quote(&mut self) {
        while self.current < self.contents.len() {
            let c = self.advance();
            match c {
                Some('\n') => {
                    self.line += 1;
                    // println!("n, {}", self.column_no);
                    self.column_no = 0;
                }
                Some('\r') => {
                    self.line += 1;
                    // println!("r, {}", self.column_no);
                    self.column_no = 0;
                }
                Some('"') => {
                    // println!("\nend of quote, {}", self.column_no);
                    return;
                }
                _ => {}
            }
        }
    }

    /// scane for an integer or a decimal
    pub(crate) fn scan_number(&mut self) {
        while self.current < self.contents.len() {
            let c = self.advance();
            match c {
                Some('.') => {} // decimal number
                _ => {
                    if let Some(n) = c {
                        if !n.is_numeric() {
                            return;
                        }
                    }
                }
            }
        }
    }

    /// identifier can contain alphanumeric and '_'
    pub(crate) fn scan_lexeme_with_underscore(&mut self) {
        while self.current < self.contents.len() {
            let c = self.advance();
            if let Some(x) = c {
                if !x.is_alphanumeric() && x != '_' {
                    return;
                }
            }
        }
    }

    /// Store the token in our scanner
    pub(crate) fn add_token(&mut self, tok_type: TokenType, literal: Option<String>) {
        let curr_str = &self.contents[self.start..self.current];
        let len_tok = self.current - self.start;
        let mut start_col_no = 0;
        if self.column_no > len_tok {
            start_col_no = self.column_no - len_tok;
        }
        self.tokens.push(Token::new(
            tok_type,
            curr_str.to_string(),
            literal,
            self.line,
            start_col_no, // we need the start column
        ));
    }

    /// Start scanning the tokens from start
    pub(crate) fn scan_tokens(&mut self) {
        // for tok in self.contents.split_ascii_whitespace().into_iter() {
        //     println!("tok: {}", tok);
        // }
        while self.current < self.contents.len() {
            self.start = self.current;
            self.scan_token();
        }
        let eof_tok = Token::new(
            TokenType::EoF,
            "".to_string(),
            None,
            self.line,
            self.column_no,
        );
        self.tokens.push(eof_tok);
    }

    /// Scan toke based on the `token_type`. For multi characters
    /// token, use `peek()`.
    /// TODO: handle errors
    pub(crate) fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // handle single character ones
            Some('(') => self.add_token(TokenType::LeftParen, None),
            Some(')') => self.add_token(TokenType::RightParen, None),
            Some('{') => self.add_token(TokenType::LeftBrace, None),
            Some('}') => {
                self.add_token(TokenType::RightBrace, None);
            }
            Some(';') => self.add_token(TokenType::SemiColon, None),
            Some(':') => self.add_token(TokenType::Colon, None),
            Some('.') => self.add_token(TokenType::Dot, None),
            Some(',') => self.add_token(TokenType::Comma, None),
            Some('*') => self.add_token(TokenType::Star, None),
            Some('-') => self.add_token(TokenType::Minus, None),
            // handle two character operators
            Some('<') => {
                // peek, if '=', then LessEqual
                // otherwise emit Equal
                match self.peek() {
                    Some('=') => {
                        self.add_token(TokenType::LessEqual, None);
                        self.current += 1;
                    }
                    _ => self.add_token(TokenType::Less, None),
                }
            }
            Some('>') => {
                // peek, if '=', then GreaterEqual
                // otherwise emit Greater
                match self.peek() {
                    Some('=') => {
                        self.add_token(TokenType::GreaterEqual, None);
                        self.current += 1;
                    }
                    _ => self.add_token(TokenType::Greater, None),
                }
            }
            Some('!') => {
                // peek, if '=', then BangEqual
                // otherwise emit bang
                match self.peek() {
                    Some('=') => {
                        self.add_token(TokenType::BangEqual, None);
                        self.current += 1;
                    }
                    _ => self.add_token(TokenType::Bang, None),
                }
            }
            Some('=') => {
                // peek, if '=', then EqualEqual
                // otherwie emit Equal
                match self.peek() {
                    Some('=') => {
                        self.add_token(TokenType::EqualEqual, None);
                        self.current += 1;
                    }
                    _ => self.add_token(TokenType::Equal, None),
                }
            }
            Some('/') => {
                // peek, if '/', then advance until \n, emit Comment
                // otherwie emit Div
                match self.peek() {
                    Some('/') => {
                        self.read_until_eol();
                        self.add_token(TokenType::Comment, None);
                        self.current += 1;
                    }
                    _ => self.add_token(TokenType::Div, None),
                }
            }
            // handle string literal
            Some('"') => {
                //TODO: lose the double quotes at both ends?
                self.read_until_eo_quote();
                let literal = &self.contents[self.start + 1..self.current - 1];
                self.add_token(TokenType::StringLiteral, Some(literal.to_string()));
                // println!("\nquote, final: {}", self.colu
                // println!(
                //     "after quote: *{}*",
                //     &self.contents[self.current - 1..self.current + 1]
                // );
                //self.current += 1;
            }
            // handle whitespace and CRLF
            Some(' ') => {}
            Some('\t') => {}
            Some('\r') => {
                self.line += 1;
                self.column_no = 1;
                // println!("main r, final: {}", self.column_no);
            }
            Some('\n') => {
                self.line += 1;
                self.column_no = 1;
                // println!("main n, final: {}", self.column_no);
            }
            _ => {
                // the rest we handle here:
                // numbers, keywords and identifiers
                // identifiers don't start with digits, so if a lexeme starts
                // with a digit, then it is a number
                // println!("other: {}", self.column_no);
                if let Some(x) = c {
                    // println!("other inside: {}", self.column_no);
                    if x.is_numeric() {
                        self.scan_number();
                        self.current -= 1;
                        let n = &self.contents[self.start..self.current];
                        self.add_token(TokenType::Number, Some(n.to_string()));
                        // self.current += 1;
                    } else {
                        // can be either an identifier or keywords
                        // if the current lexeme doesn't start with a digit, then
                        // it must be either an identifier or a keyword.
                        // identifiers can contain digits as long as they don't
                        // start with a digit.
                        self.scan_lexeme_with_underscore();
                        let s = &self.contents[self.start..self.current - 1];
                        // println!("s: {}", s);
                        if KEYWORDS.contains(&s) {
                            self.current -= 1;
                            // println!("==> adding keyword!");
                            self.add_token(TokenType::Keyword, Some(s.to_string()));
                            self.current += 1;
                        } else {
                            self.current -= 1;
                            // println!("==> adding identifier!");
                            self.add_token(TokenType::Identifier, Some(s.to_string()));
                            self.current += 1;
                        }
                    }
                }
            }
        }
    }
}

/// Report an error using the line number and a short message
pub(crate) fn error(line: u32, msg: &str) {
    // also add to errors
    report(line, "".to_string().as_str(), &msg);
}

/// Print the error message
pub(crate) fn report(line: u32, col_loc: &str, msg: &str) {
    println!("[line [{}] Error where: {}: {}] ", line, col_loc, msg);
}

/// Read the input file and return a `Scanner`
pub(crate) fn read_lex_file(s: &str) -> Result<Scanner, Box<dyn error::Error>> {
    let lexf_s = fs::read_to_string(s)?;
    let my_scanner: Scanner = Scanner::new(s.to_string(), lexf_s.to_string());
    Ok(my_scanner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let s = "aws { ec2 { count = 10 } }";
        let mut scanr = Scanner::new("".to_string(), s.to_string());
        scanr.scan_tokens();
        let number_tok_exists = |tokens: Vec<Token>| -> bool {
            for tok in tokens {
                if tok.token_type == TokenType::Number {
                    return true;
                }
            }
            false
        };
        assert!(number_tok_exists(scanr.tokens));
    }

    #[test]
    fn test_decimal_number() {
        let s = "aws { ec2 { count = 10.0 } }";
        let mut scanr = Scanner::new("".to_string(), s.to_string());
        scanr.scan_tokens();
        let num_tokens = scanr
            .tokens
            .iter()
            .filter_map(|tok| match tok.token_type {
                TokenType::Number => tok.lexeme.parse::<f32>().ok(),
                _ => None,
            })
            .collect::<Vec<f32>>();
        assert_eq!(num_tokens.get(0), Some(&10.0));
    }

    #[test]
    fn test_string() {
        let s = "aws { ec2 { name = \"my_node\" } }";
        let mut scanr = Scanner::new("".to_string(), s.to_string());
        scanr.scan_tokens();
        let comment_toks = scanr
            .tokens
            .iter()
            .filter_map(|tok| match tok.token_type {
                TokenType::StringLiteral => Some(tok.literal.as_ref().unwrap()),
                _ => None,
            })
            .collect::<Vec<&String>>();
        let expected = String::from("my_node");
        assert_eq!(comment_toks.get(0), Some(&&expected));
    }

    #[test]
    fn test_ec2_id() {
        let s = "aws { ec2 { ec2_id = 10 } }";
        let mut scanr = Scanner::new("".to_string(), s.to_string());
        scanr.scan_tokens();
        let kword_tokens = scanr
            .tokens
            .iter()
            .filter_map(|tok| match tok.token_type {
                TokenType::Keyword => Some(tok.lexeme.as_str()),
                _ => None,
            })
            .collect::<Vec<&str>>();
        // println!("kword_tokens: {:?}", kword_tokens);
        assert_eq!(kword_tokens.get(2), Some(&"ec2_id"));
    }

    #[test]
    fn test_image() {
        let s = "aws { ec2 { image = \"test-image\" } }";
        let mut scanr = Scanner::new("".to_string(), s.to_string());
        scanr.scan_tokens();
        let kword_tokens = scanr
            .tokens
            .iter()
            .filter_map(|tok| match tok.token_type {
                TokenType::Keyword => Some(tok.lexeme.as_str()),
                _ => None,
            })
            .collect::<Vec<&str>>();
        assert_eq!(kword_tokens.get(2), Some(&"image"));
    }

    #[test]
    fn test_app_version() {
        let s = "aws { ec2 { app_version = 10.0 } }";
        let mut scanr = Scanner::new("".to_string(), s.to_string());
        scanr.scan_tokens();
        let kword_tokens = scanr
            .tokens
            .iter()
            .filter_map(|tok| match tok.token_type {
                TokenType::Keyword => Some(tok.lexeme.as_str()),
                _ => None,
            })
            .collect::<Vec<&str>>();
        assert_eq!(kword_tokens.get(2), Some(&"app_version"));
    }

    #[test]
    fn test_comment() {
        //
    }
}
