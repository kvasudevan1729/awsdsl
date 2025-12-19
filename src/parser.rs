use std::fmt::{self};

use crate::lex::{Token, TokenType};
use crate::nodes::{AwsNode, Ec2Node};

#[derive(Debug)]
pub(crate) struct ParseError {
    err_type: String,
    message: String,
}

impl ParseError {
    pub(crate) fn new(err_type: String, msg: String) -> Self {
        ParseError {
            err_type: err_type,
            message: msg,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error type: {}, mesg: {}", self.err_type, self.message)
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

    /// parse starts from aws block
    pub(crate) fn parse(&mut self) -> Result<AwsNode, ParseError> {
        println!("==> parsing ..");
        match self.next() {
            Some(n) => {
                if n.lexeme == "aws" {
                    self.aws()
                } else {
                    Err(ParseError::new(
                        "aws_error".to_string(),
                        "Not an aws constuct".to_string(),
                    ))
                }
            }
            _ => Err(ParseError {
                err_type: "aws_error".to_string(),
                message: "Not an aws constuct".to_string(),
            }),
        }
    }

    /// check if the current pointer matches with token provided
    fn check_token(&mut self, tok_type: TokenType) -> Result<(), ParseError> {
        if let Some(n) = self.peek() {
            println!("tok_type: {}", tok_type);
            if n.token_type == tok_type {
                self.current += 1;
                return Ok(());
            }
        }
        let s = format!(
            "token error:: expecting {}, found token: {}",
            tok_type,
            self.peek().unwrap()
        );
        Err(ParseError::new("token error".to_string(), s))
    }

    /// Parse aws block
    fn aws(&mut self) -> Result<AwsNode, ParseError> {
        let mut aws_node = AwsNode::new("aws".to_string());
        println!(
            "-- aws[{}:{}] --",
            self.current,
            self.tokens.get(self.current).unwrap()
        );
        self.check_token(TokenType::LeftBrace)?;
        println!("-- after leftbrace: {}", self.current);
        // parse attributes - name, region, ec2 ...
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
                        let s = format!("Unexpected keyword: {}", aws_attr.lexeme);
                        return Err(ParseError::new("Unexpected keyword".to_string(), s));
                    }
                },
                TokenType::RightBrace => {
                    return Ok(aws_node);
                }
                _ => return Err(ParseError::new("".to_string(), "".to_string())),
            }
        }

        Err(ParseError::new("".to_string(), "".to_string()))
    }

    fn aws_name(&mut self, aws_node: &mut AwsNode) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    println!("name: {}", tok.lexeme);
                    aws_node.set_name(tok.lexeme.to_string());
                    return Ok(());
                }
                _ => {}
            }
        }
        Err(ParseError::new(
            "token error".to_string(),
            "expecting aws name!".to_string(),
        ))
    }

    fn aws_description(&mut self, aws_node: &mut AwsNode) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    println!("desc: {}", tok.lexeme);
                    aws_node.set_description(tok.lexeme.to_string());
                    return Ok(());
                }
                _ => {}
            }
        }
        Err(ParseError::new(
            "token error".to_string(),
            "expecting aws description!".to_string(),
        ))
    }

    fn aws_region(&mut self, aws_node: &mut AwsNode) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    println!("region: {}", tok.lexeme);
                    aws_node.set_region(tok.lexeme.to_string());
                    return Ok(());
                }
                _ => {}
            }
        }
        Err(ParseError::new(
            "bad region".to_string(),
            "bad region".to_string(),
        ))
    }

    fn ec2(&mut self, aws_node: &mut AwsNode) -> Result<(), ParseError> {
        println!("ec2: {}", self.peek().unwrap());
        self.check_token(TokenType::LeftBrace)?;
        println!("***  -- after leftbrace: {}", self.current);
        let mut ec2 = Ec2Node::new("my-ec2".to_string());

        while let Some(ec2_attr) = self.next() {
            match ec2_attr.token_type {
                TokenType::Keyword => match ec2_attr.lexeme.as_str() {
                    "name" => {
                        self.ec2_name(&mut ec2)?;
                    }
                    "description" => {
                        self.ec2_description(&mut ec2)?;
                    }
                    "count" => {
                        self.ec2_count(&mut ec2)?;
                    }
                    "image" => {
                        self.ec2_image(&mut ec2)?;
                    }
                    "app_version" => {
                        self.ec2_app_version(&mut ec2)?;
                    }
                    _ => {
                        let s = format!("Unexpected keyword: {}", ec2_attr.lexeme);
                        return Err(ParseError::new("Unexpected keyword".to_string(), s));
                    }
                },
                TokenType::Comment => {}
                TokenType::RightBrace => {
                    aws_node.add_ec2(ec2);
                    return Ok(());
                }
                _ => {
                    return Err(ParseError::new(
                        "token error".to_string(),
                        "Unexpected tokens within ec2".to_string(),
                    ));
                }
            }
        }

        Err(ParseError::new(
            "token error".to_string(),
            "Unexpected ec2".to_string(),
        ))
    }

    fn ec2_name(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    println!("name: {}", tok.lexeme);
                    ec2_node.set_name(tok.lexeme.to_string());
                    return Ok(());
                }
                _ => {}
            }
        }
        Err(ParseError::new(
            "token error".to_string(),
            "expecting ec2 name!".to_string(),
        ))
    }

    fn ec2_description(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    println!("desc: {}", tok.lexeme);
                    ec2_node.set_description(tok.lexeme.to_string());
                    return Ok(());
                }
                _ => {}
            }
        }
        Err(ParseError::new(
            "token error".to_string(),
            "expecting ec2 description!".to_string(),
        ))
    }

    fn ec2_image(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::StringLiteral => {
                    println!("image: {}", tok.lexeme);
                    ec2_node.set_image(tok.lexeme.to_string());
                    return Ok(());
                }
                _ => {}
            }
        }
        Err(ParseError::new(
            "token error".to_string(),
            "expecting ec2 image!".to_string(),
        ))
    }

    fn ec2_count(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Number => {
                    println!("count: {}", tok.lexeme);
                    ec2_node.set_count(tok.lexeme.parse::<u8>().unwrap_or_default());
                    return Ok(());
                }
                _ => {}
            }
        }
        Err(ParseError::new(
            "token error".to_string(),
            "expecting ec2 count!".to_string(),
        ))
    }

    fn ec2_app_version(&mut self, ec2_node: &mut Ec2Node) -> Result<(), ParseError> {
        self.check_token(TokenType::Equal)?;
        if let Some(tok) = self.next() {
            match tok.token_type {
                TokenType::Number => {
                    println!("app_version: {}", tok.lexeme);
                    ec2_node.set_app_version(tok.lexeme.parse::<f32>().unwrap_or_default());
                    return Ok(());
                }
                _ => {}
            }
        }
        Err(ParseError::new(
            "token error".to_string(),
            "expecting ec2 count!".to_string(),
        ))
    }
}
