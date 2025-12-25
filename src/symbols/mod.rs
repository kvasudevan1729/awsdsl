use std::fmt;

use crate::aws::{AwsSym, Ec2Sym, nodes};

pub(crate) struct AstError {
    msg: String,
}

impl AstError {
    pub(crate) fn new(msg: impl std::convert::Into<String>) -> Self {
        Self { msg: msg.into() }
    }
}

impl fmt::Display for AstError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ast Error: {}", self.msg)
    }
}

pub(crate) fn walk_ast(aws_node: &nodes::AwsNode) -> Result<AwsSym, AstError> {
    println!("==> walking the aws ast ...");
    let aws_region = aws_node
        .region
        .as_ref()
        .ok_or(AstError::new("No region provided!"))?;
    let mut aws_sym = AwsSym::new(aws_node.id.to_string(), String::from(aws_region));
    for ec2 in &aws_node.ec2_nodes {
        let ec2_sym = Ec2Sym::new(
            String::from(
                ec2.id
                    .as_ref()
                    .ok_or(AstError::new("No ec2 description provided"))?,
            ),
            String::from(
                ec2.description
                    .as_ref()
                    .ok_or(AstError::new("No ec2 description provided"))?,
            ),
            String::from(
                ec2.instance_type
                    .as_ref()
                    .ok_or(AstError::new("No ec2 instance type provided"))?,
            ),
            String::from(
                ec2.ami
                    .as_ref()
                    .ok_or(AstError::new("No ec2 ami id provided!"))?,
            ),
            String::from(
                ec2.subnet_id
                    .as_ref()
                    .ok_or(AstError::new("No ec2 subnet_id provided!"))?,
            ),
            String::from(
                ec2.sg_id
                    .as_ref()
                    .ok_or(AstError::new("No ec2 sg id provided!"))?,
            ),
            ec2.app_version,
            ec2.count,
        );
        aws_sym.add_ec2(ec2_sym);
    }

    Ok(aws_sym)
}
