use std::error::Error;
use std::fmt;

use crate::lex::Token;

pub(crate) struct AwsNode {
    id: String,
    ec2_node: Option<Ec2Node>,
}

impl AwsNode {
    pub(crate) fn new(id: String) -> Self {
        AwsNode {
            id: id,
            ec2_node: None,
        }
    }

    pub(crate) fn add_ec2(&mut self, ec2: Ec2Node) {
        self.ec2_node = Some(ec2);
    }
}

impl fmt::Display for AwsNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]{}", self.id, self.ec2_node.as_ref().unwrap())
    }
}

pub(crate) struct Ec2Node {
    id: String,
    ec2_attrs: Vec<Ec2AttrNode>,
}

impl Ec2Node {
    pub(crate) fn new(id: String) -> Self {
        Ec2Node {
            id: id,
            ec2_attrs: vec![],
        }
    }

    pub(crate) fn add_addr(&mut self, attr: Ec2AttrNode) {
        self.ec2_attrs.push(attr);
    }
}

impl fmt::Display for Ec2Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for attr in &self.ec2_attrs {
            s = format!("{}\n{}", s, attr);
        }
        write!(f, "\n  [{}]{}", self.id, s)
    }
}

pub(crate) struct Ec2AttrNode {
    id: String,
}

impl Ec2AttrNode {
    pub(crate) fn new(id: String) -> Self {
        Ec2AttrNode { id: id }
    }
}

impl fmt::Display for Ec2AttrNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "    [{}]", self.id)
    }
}

pub(crate) struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

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

impl<'a> Parser<'a> {
    pub(crate) fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    // peeks the next token but doesn't move the pointer
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    // Retreives the next token and moves pointer
    fn next(&mut self) -> Option<&Token> {
        let next_tok = self.tokens.get(self.current);
        self.current += 1;
        next_tok
    }

    // parsing starts from aws block
    pub(crate) fn parse(&mut self) -> Result<AwsNode, ParseError> {
        println!("==> parsing aws code ..");
        match self.peek() {
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

    // Parse the aws block and build parse tree
    fn aws(&mut self) -> Result<AwsNode, ParseError> {
        if let Some(aws) = self.next() {
            let mut aws_node = AwsNode::new(aws.lexeme.to_string());
            // attrs - name, region

            let ec2 = self.ec2()?;
            aws_node.add_ec2(ec2);
            return Ok(aws_node);
        }

        Err(ParseError::new("".to_string(), "".to_string()))
    }

    fn ec2(&mut self) -> Result<Ec2Node, ParseError> {
        let mut ec2 = Ec2Node::new("my-ec2".to_string());
        let ec2_name_attr = Ec2AttrNode::new("ec2_name".to_string());
        ec2.add_addr(ec2_name_attr);
        Ok(ec2)
    }
}
