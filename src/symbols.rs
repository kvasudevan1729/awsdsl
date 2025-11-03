use std::fmt;

use crate::lex::Scanner;

type AwsRegion = String;

struct AwsStack {
    name: String,
    description: String,
    region: AwsRegion,
    ec2_hosts: Vec<Ec2>,
}

impl AwsStack {
    pub(crate) fn new(name: String, description: String, region: String) -> Self {
        AwsStack {
            name: name,
            description: description,
            region: region,
            ec2_hosts: vec![],
        }
    }

    pub(crate) fn add_ec2(&mut self, ec2: Ec2) {
        self.ec2_hosts.push(ec2);
    }
}

impl fmt::Display for AwsStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]{}", self.region, self.name)
    }
}

struct Ec2 {
    id: String,
    name: String,
    description: String,
    image: String,
}

impl Ec2 {
    pub(crate) fn new(id: String, name: String, description: String, image: String) -> Self {
        Ec2 {
            id: id,
            name: name,
            description: description,
            image: image,
        }
    }
}

impl fmt::Display for Ec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub(crate) fn build_parse_tree(scanner: &Scanner) {
    println!("===> tokens ");
    for tok in &scanner.tokens {
        println!("{} ", tok);
    }
    println!("===");
    let mut aws_stack = AwsStack::new(
        String::from("my_aws"),
        String::from("my aws"),
        String::from("eu-west-1") as AwsRegion,
    );
    let test_ec2: Ec2 = Ec2::new(
        String::from("my_ec2_id"),
        String::from("my_ec2"),
        String::from("my ec2"),
        String::from("ubuntu 25.01 lts"),
    );
    aws_stack.add_ec2(test_ec2);
    println!("aws stack: {}", aws_stack);
}
