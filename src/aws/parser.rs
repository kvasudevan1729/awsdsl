use std::error::Error;
use std::fmt;

use crate::aws::nodes::{AwsNode, Ec2Node};
use crate::lex::{Scanner, Token, TokenType};

#[derive(Debug)]
pub(crate) enum ParseErrorType {
    TokenMismatch,
    UnknownToken,
}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TokenMismatch => "Token mismatch",
            Self::UnknownToken => "Unknown token",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub(crate) struct ParseError {
    err_type: ParseErrorType,
    msg: String,
}

impl Error for ParseError {}

impl ParseError {
    pub(crate) fn new(err_type: ParseErrorType, msg: impl std::convert::Into<String>) -> Self {
        ParseError {
            err_type: err_type,
            msg: msg.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error type: {}, details: {}", self.err_type, self.msg)
    }
}

pub(crate) struct Parser {
    scanner: Scanner,
}

impl Parser {
    pub(crate) fn new(scanner: Scanner) -> Self {
        Parser { scanner: scanner }
    }

    /// Peeks into the current token but doesn't move the pointer
    // The scanner.start has not moved (as it will be reset only
    // at the next_token call). So we reset current to the start
    // [OR] another possibility: don't go all the way back but just
    // by word length.
    // fn peek(&mut self) -> Option<Token> {
    //     println!("== inside peek()");
    //     if let Some(tok) = self.next() {
    //         let tok_len = tok.lexeme.len();
    //         self.scanner.current = self.scanner.start;
    //         println!(
    //             "start: {}, current: {}",
    //             self.scanner.start, self.scanner.current
    //         );
    //         println!(
    //             "[len]start: {}, current: {}, len: {}",
    //             self.scanner.start, self.scanner.current, tok_len
    //         );
    //         return Some(tok.clone());
    //     }
    //
    //     return None;
    // }

    /// Retrieves the next lexeme from the scanner
    fn next(&mut self) -> Option<Token> {
        let next_tok = self.scanner.next_token();
        Some(next_tok)
    }

    /// parse starts from aws block
    pub(crate) fn parse(&mut self) -> Result<AwsNode, ParseError> {
        println!("==> parsing ...");
        if let Some(n) = self.next() {
            if n.token_type == TokenType::EoF {
                let s = format!("Expecting program to begin with aws block!");
                return Err(ParseError::new(ParseErrorType::TokenMismatch, s));
            }
            if n.lexeme == "aws" {
                return self.aws();
            }
        }

        let s = format!("Unknown literal, should start from aws block!");
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }

    /// Parse aws block, expects one of name, description, region or ec2 block
    pub(crate) fn aws(&mut self) -> Result<AwsNode, ParseError> {
        println!("== inside aws()");
        let mut aws_node = AwsNode::new("aws".to_string());
        while let Some(aws_attr) = self.next() {
            match aws_attr.token_type {
                TokenType::Keyword => match aws_attr.lexeme.as_str() {
                    "region" => {
                        self.aws_region(&mut aws_node)?;
                    }
                    "name" => {
                        self.aws_name(&mut aws_node)?;
                    }
                    "description" => {
                        self.aws_description(&mut aws_node)?;
                    }
                    "ec2" => {
                        self.ec2(&mut aws_node)?;
                    }
                    _ => {
                        let s = format!(
                            "Invalid token {} at location ({},{})",
                            aws_attr.lexeme, aws_attr.line_no, aws_attr.column_no
                        );
                        return Err(ParseError::new(ParseErrorType::UnknownToken, s));
                    }
                },
                TokenType::LeftBrace => {}
                TokenType::RightBrace => {
                    return Ok(aws_node);
                }
                _ => {
                    let s = format!(
                        "Invalid token {} at location ({},{})",
                        aws_attr.lexeme, aws_attr.line_no, aws_attr.column_no
                    );
                    return Err(ParseError::new(ParseErrorType::UnknownToken, s));
                }
            }
        }

        let s = format!(
            "Invalid token at (line,column): {},{}",
            self.scanner.line, self.scanner.column_no
        );
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }

    /// parse the name statement after `name` keyword
    fn aws_name(&mut self, aws_node: &mut AwsNode) -> Result<(), ParseError> {
        while let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Equal => {}
                TokenType::StringLiteral => {
                    aws_node.set_name(tok.lexeme.trim_matches('"'));
                    return Ok(());
                }
                _ => {
                    let s = format!(
                        "Invalid token at location ({},{}), found: {}",
                        tok.line_no, tok.column_no, tok.lexeme
                    );
                    return error(&&self.scanner, ParseErrorType::TokenMismatch, Some(s));
                }
            }
        }

        error(&self.scanner, ParseErrorType::TokenMismatch, None)
    }

    fn aws_description(&mut self, aws_node: &mut AwsNode) -> Result<(), ParseError> {
        while let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Equal => {}
                TokenType::StringLiteral => {
                    aws_node.set_description(tok.lexeme.trim_matches('"'));
                    return Ok(());
                }
                _ => {
                    let s = format!(
                        "Invalid token at location ({},{}), found: {}",
                        tok.line_no, tok.column_no, tok.lexeme
                    );
                    return error(&&self.scanner, ParseErrorType::TokenMismatch, Some(s));
                }
            }
        }

        error(&self.scanner, ParseErrorType::TokenMismatch, None)
    }

    fn aws_region(&mut self, aws_node: &mut AwsNode) -> Result<(), ParseError> {
        while let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Equal => {}
                TokenType::StringLiteral => {
                    aws_node.set_region(tok.lexeme.trim_matches('"'));
                    return Ok(());
                }
                _ => {
                    let s = format!(
                        "Invalid token at location ({},{}), found: {}",
                        tok.line_no, tok.column_no, tok.lexeme
                    );
                    return error(&&self.scanner, ParseErrorType::TokenMismatch, Some(s));
                }
            }
        }

        error(&self.scanner, ParseErrorType::TokenMismatch, None)
    }

    fn ec2(&mut self, aws_node: &mut AwsNode) -> Result<(), ParseError> {
        let mut ec2 = Ec2Node::new();

        while let Some(ec2_attr) = self.next() {
            match ec2_attr.token_type {
                TokenType::Keyword => match ec2_attr.lexeme.as_str() {
                    "name" => {
                        self.ec2_name(&mut ec2)?;
                    }
                    "description" => {
                        self.ec2_description(&mut ec2)?;
                    }
                    "instance_type" => {
                        self.ec2_instance_type(&mut ec2)?;
                    }
                    "count" => {
                        self.ec2_count(&mut ec2)?;
                    }
                    "ami" => {
                        self.ec2_ami(&mut ec2)?;
                    }
                    "subnet_id" => {
                        self.ec2_subnet_id(&mut ec2)?;
                    }
                    "sg_id" => {
                        self.ec2_sg_id(&mut ec2)?;
                    }
                    "app_version" => {
                        self.ec2_app_version(&mut ec2)?;
                    }
                    "key_name" => {
                        self.ec2_key_name(&mut ec2)?;
                    }
                    _ => {
                        let s = format!(
                            "Invalid token {} at location ({},{})",
                            ec2_attr.lexeme, ec2_attr.line_no, ec2_attr.column_no
                        );
                        return Err(ParseError::new(ParseErrorType::UnknownToken, s));
                    }
                },
                TokenType::Comment => {}
                TokenType::LeftBrace => {}
                TokenType::RightBrace => {
                    aws_node.add_ec2(ec2);
                    return Ok(());
                }
                _ => {
                    let s = format!(
                        "Invalid token {} at location ({},{})",
                        ec2_attr.lexeme, ec2_attr.line_no, ec2_attr.column_no
                    );
                    return Err(ParseError::new(ParseErrorType::UnknownToken, s));
                }
            }
        }

        error(&self.scanner, ParseErrorType::TokenMismatch, None)
    }

    fn ec2_name(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        while let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Equal => {}
                TokenType::StringLiteral => {
                    ec2_node.set_name(tok.lexeme.trim_matches('"'));
                    return Ok(());
                }
                _ => {
                    let s = format!(
                        "Invalid token at location ({},{}), found: {}",
                        tok.line_no, tok.column_no, tok.lexeme
                    );
                    return error(&&self.scanner, ParseErrorType::TokenMismatch, Some(s));
                }
            }
        }

        error(&self.scanner, ParseErrorType::TokenMismatch, None)
    }

    fn ec2_description(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        while let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Equal => {}
                TokenType::StringLiteral => {
                    ec2_node.set_description(tok.lexeme.trim_matches('"'));
                    return Ok(());
                }
                _ => {
                    let s = format!(
                        "Invalid token at location ({},{}), found: {}",
                        tok.line_no, tok.column_no, tok.lexeme
                    );
                    return error(&&self.scanner, ParseErrorType::TokenMismatch, Some(s));
                }
            }
        }

        error(&self.scanner, ParseErrorType::TokenMismatch, None)
    }

    fn ec2_instance_type(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        while let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Equal => {}
                TokenType::StringLiteral => {
                    ec2_node.set_instance_type(tok.lexeme.trim_matches('"'));
                    return Ok(());
                }
                _ => {
                    let s = format!(
                        "Invalid token at location ({},{}), found: {}",
                        tok.line_no, tok.column_no, tok.lexeme
                    );
                    return error(&&self.scanner, ParseErrorType::TokenMismatch, Some(s));
                }
            }
        }

        error(&self.scanner, ParseErrorType::TokenMismatch, None)
    }

    fn ec2_ami(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        while let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Equal => {}
                TokenType::StringLiteral => {
                    ec2_node.set_ami(tok.lexeme.trim_matches('"'));
                    return Ok(());
                }
                _ => {
                    let s = format!(
                        "Invalid token at location ({},{}), found: {}",
                        tok.line_no, tok.column_no, tok.lexeme
                    );
                    return error(&&self.scanner, ParseErrorType::TokenMismatch, Some(s));
                }
            }
        }

        error(&self.scanner, ParseErrorType::TokenMismatch, None)
    }

    fn ec2_subnet_id(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        while let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Equal => {}
                TokenType::StringLiteral => {
                    ec2_node.set_subnet_id(tok.lexeme.trim_matches('"'));
                    return Ok(());
                }
                _ => {
                    let s = format!(
                        "Invalid token at location ({},{}), found: {}",
                        tok.line_no, tok.column_no, tok.lexeme
                    );
                    return error(&&self.scanner, ParseErrorType::TokenMismatch, Some(s));
                }
            }
        }

        error(&self.scanner, ParseErrorType::TokenMismatch, None)
    }

    fn ec2_sg_id(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        while let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Equal => {}
                TokenType::StringLiteral => {
                    ec2_node.set_sg_id(tok.lexeme.trim_matches('"'));
                    return Ok(());
                }
                _ => {
                    let s = format!(
                        "Invalid token at location ({},{}), found: {}",
                        tok.line_no, tok.column_no, tok.lexeme
                    );
                    return error(&&self.scanner, ParseErrorType::TokenMismatch, Some(s));
                }
            }
        }

        error(&self.scanner, ParseErrorType::TokenMismatch, None)
    }

    fn ec2_count(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        while let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Equal => {}
                TokenType::Number => {
                    ec2_node.set_count(tok.lexeme.parse::<u8>().unwrap_or_default());
                    return Ok(());
                }
                _ => {
                    let s = format!(
                        "Invalid token at location ({},{}), found: {}",
                        tok.line_no, tok.column_no, tok.lexeme
                    );
                    return error(&&self.scanner, ParseErrorType::TokenMismatch, Some(s));
                }
            }
        }

        error(&self.scanner, ParseErrorType::TokenMismatch, None)
    }

    fn ec2_app_version(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        while let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Equal => {}
                TokenType::Number => {
                    ec2_node.set_app_version(tok.lexeme.parse::<f32>().unwrap_or_default());
                    return Ok(());
                }
                _ => {
                    let s = format!(
                        "Invalid token at location ({},{}), found: {}",
                        tok.line_no, tok.column_no, tok.lexeme
                    );
                    return error(&&self.scanner, ParseErrorType::TokenMismatch, Some(s));
                }
            }
        }

        error(&self.scanner, ParseErrorType::TokenMismatch, None)
    }

    fn ec2_key_name(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        while let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Equal => {}
                TokenType::StringLiteral => {
                    ec2_node.set_key_name(tok.lexeme.trim_matches('"'));
                    return Ok(());
                }
                _ => {
                    let s = format!(
                        "Invalid token at location ({},{}), found: {}",
                        tok.line_no, tok.column_no, tok.lexeme
                    );
                    return error(&&self.scanner, ParseErrorType::TokenMismatch, Some(s));
                }
            }
        }

        error(&self.scanner, ParseErrorType::TokenMismatch, None)
    }
}

pub(crate) fn error(
    scanr: &Scanner,
    err_type: ParseErrorType,
    msg: Option<String>,
) -> Result<(), ParseError> {
    match msg {
        Some(s) => Err(ParseError::new(err_type, s)),
        _ => {
            let s = format!(
                "Invalid token at (line,column): {},{}",
                scanr.line, scanr.column_no
            );
            Err(ParseError::new(err_type, s))
        }
    }
}
