use std::fmt;

use crate::aws::nodes::{AwsNode, Ec2Node};
use crate::lex::{Token, TokenType};

pub(crate) enum ParseErrorType {
    TokenMismatch,
    UnknownToken,
}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = match self {
            Self::TokenMismatch => String::from("Token mismatch"),
            Self::UnknownToken => String::from("Unknown token"),
        };
        write!(f, "{}", s)
    }
}

pub(crate) struct ParseError {
    err_type: ParseErrorType,
    msg: String,
}

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

pub(crate) struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    /// peeks into the current token but doesn't move the pointer
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    /// Retrieves the current token and moves the pointer by 1.
    fn next(&mut self) -> Option<&Token> {
        let next_tok = self.tokens.get(self.current);
        self.current += 1;
        next_tok
    }

    /// check if the current pointer matches with token provided
    fn check_token(&mut self, tok_type: TokenType) -> Result<(), ParseError> {
        if let Some(n) = self.peek() {
            if n.token_type == tok_type {
                self.current += 1;
                return Ok(());
            }
        }
        let tok = self.peek().unwrap();
        let s = format!(
            "Expecting {} at location ({},{}, but found: {}",
            tok_type, tok.line_no, tok.column_no, tok.lexeme
        );
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }

    /// parse starts from aws block
    pub(crate) fn parse(&mut self) -> Result<AwsNode, ParseError> {
        println!("==> parse the tokens ...");
        if let Some(n) = self.next() {
            if n.lexeme == "aws" {
                return self.aws();
            }
        }

        let tok = self.peek().unwrap();
        let s = format!(
            "Expecting {} at location ({},{}, but found: {}",
            "aws", tok.line_no, tok.column_no, tok.lexeme
        );
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }

    /// Parse aws block, expects one of name, description, region or ec2 block
    pub(crate) fn aws(&mut self) -> Result<AwsNode, ParseError> {
        let mut aws_node = AwsNode::new("aws".to_string());
        self.check_token(TokenType::LeftBrace)?;
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
        let tok = self.peek().unwrap();
        let s = format!(
            "Error parsing token {} at location ({},{})",
            tok.lexeme, tok.line_no, tok.column_no
        );
        return Err(ParseError::new(ParseErrorType::UnknownToken, s));
    }

    /// parse the name statement after `name` keyword
    fn aws_name(&mut self, aws_node: &mut AwsNode) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    aws_node.set_name(tok.lexeme.trim_matches('"'));
                    return Ok(());
                }
                _ => {}
            }
        }

        let c_tok = self.peek().unwrap();
        let s = format!(
            "Expecting a String literal token at location ({},{}), but found: {}",
            c_tok.line_no, c_tok.column_no, c_tok.lexeme
        );
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }

    fn aws_description(&mut self, aws_node: &mut AwsNode) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    aws_node.set_description(tok.lexeme.trim_matches('"'));
                    return Ok(());
                }
                _ => {}
            }
        }
        let c_tok = self.peek().unwrap();
        let s = format!(
            "Expecting a String literal token at location ({},{}), but found: {}",
            c_tok.line_no, c_tok.column_no, c_tok.lexeme
        );
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }

    fn aws_region(&mut self, aws_node: &mut AwsNode) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    aws_node.set_region(tok.lexeme.trim_matches('"'));
                    return Ok(());
                }
                _ => {}
            }
        }
        let c_tok = self.peek().unwrap();
        let s = format!(
            "Expecting a String literal token at location ({},{}), but found: {}",
            c_tok.line_no, c_tok.column_no, c_tok.lexeme
        );
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }

    fn ec2(&mut self, aws_node: &mut AwsNode) -> Result<(), ParseError> {
        self.check_token(TokenType::LeftBrace)?;
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
                    _ => {
                        let s = format!(
                            "Invalid token {} at location ({},{})",
                            ec2_attr.lexeme, ec2_attr.line_no, ec2_attr.column_no
                        );
                        return Err(ParseError::new(ParseErrorType::UnknownToken, s));
                    }
                },
                TokenType::Comment => {}
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

        let tok = self.peek().unwrap();
        let s = format!(
            "Error parsing token {} at location ({},{})",
            tok.lexeme, tok.line_no, tok.column_no
        );
        return Err(ParseError::new(ParseErrorType::UnknownToken, s));
    }

    fn ec2_name(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    ec2_node.set_name(tok.lexeme.trim_matches('"').to_string());
                    return Ok(());
                }
                _ => {}
            }
        }
        let c_tok = self.peek().unwrap();
        let s = format!(
            "Expecting a String literal token at location ({},{}), but found: {}",
            c_tok.line_no, c_tok.column_no, c_tok.lexeme
        );
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }

    fn ec2_description(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    ec2_node.set_description(tok.lexeme.trim_matches('"').to_string());
                    return Ok(());
                }
                _ => {}
            }
        }
        let c_tok = self.peek().unwrap();
        let s = format!(
            "Expecting a String literal token at location ({},{}), but found: {}",
            c_tok.line_no, c_tok.column_no, c_tok.lexeme
        );
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }

    fn ec2_instance_type(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    ec2_node.set_instance_type(tok.lexeme.trim_matches('"').to_string());
                    return Ok(());
                }
                _ => {}
            }
        }
        let c_tok = self.peek().unwrap();
        let s = format!(
            "Expecting a String literal token at location ({},{}), but found: {}",
            c_tok.line_no, c_tok.column_no, c_tok.lexeme
        );
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }

    fn ec2_ami(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    ec2_node.set_ami(tok.lexeme.trim_matches('"').to_string());
                    return Ok(());
                }
                _ => {}
            }
        }
        let c_tok = self.peek().unwrap();
        let s = format!(
            "Expecting a String literal token at location ({},{}), but found: {}",
            c_tok.line_no, c_tok.column_no, c_tok.lexeme
        );
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }

    fn ec2_subnet_id(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    ec2_node.set_subnet_id(tok.lexeme.trim_matches('"').to_string());
                    return Ok(());
                }
                _ => {}
            }
        }
        let c_tok = self.peek().unwrap();
        let s = format!(
            "Expecting a String literal token at location ({},{}), but found: {}",
            c_tok.line_no, c_tok.column_no, c_tok.lexeme
        );
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }

    fn ec2_sg_id(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    ec2_node.set_sg_id(tok.lexeme.trim_matches('"').to_string());
                    return Ok(());
                }
                _ => {}
            }
        }
        let c_tok = self.peek().unwrap();
        let s = format!(
            "Expecting a String literal token at location ({},{}), but found: {}",
            c_tok.line_no, c_tok.column_no, c_tok.lexeme
        );
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }

    fn ec2_count(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Number => {
                    ec2_node.set_count(tok.lexeme.parse::<u8>().unwrap_or_default());
                    return Ok(());
                }
                _ => {}
            }
        }
        let c_tok = self.peek().unwrap();
        let s = format!(
            "Expecting a String literal token at location ({},{}), but found: {}",
            c_tok.line_no, c_tok.column_no, c_tok.lexeme
        );
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }

    fn ec2_app_version(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Number => {
                    ec2_node.set_app_version(tok.lexeme.parse::<f32>().unwrap_or_default());
                    return Ok(());
                }
                _ => {}
            }
        }
        let c_tok = self.peek().unwrap();
        let s = format!(
            "Expecting a String literal token at location ({},{}), but found: {}",
            c_tok.line_no, c_tok.column_no, c_tok.lexeme
        );
        Err(ParseError::new(ParseErrorType::TokenMismatch, s))
    }
}
