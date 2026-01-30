pub(crate) mod instances;

use crate::AwsSym;
use crate::actions;
use aws_config::BehaviorVersion;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::types::InstanceType;
use std::error::Error;
use std::fmt;

const AWS_REGION: &'static str = "eu-west-1";

#[derive(Debug)]
pub(crate) enum AwsErrorType {
    EC2Deploy,
}

impl fmt::Display for AwsErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::EC2Deploy => "ec2 deploy",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub(crate) struct AwsDeployError {
    err_type: AwsErrorType,
    message: String,
}

impl Error for AwsDeployError {}

impl AwsDeployError {
    pub(crate) fn new(err_type: AwsErrorType, msg: impl std::convert::Into<String>) -> Self {
        AwsDeployError {
            err_type: err_type,
            message: msg.into(),
        }
    }
    pub(crate) fn show(&self) -> String {
        format!("[{}] {}", self.err_type, self.message)
    }
}

impl fmt::Display for AwsDeployError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error type: {:?}, msg: {}", self.err_type, self.message)
    }
}

/// Supported Instance Types:
/// t2.micro, t3.small. Defaults to t2.nano
fn get_instance_type(inst_type: &str) -> InstanceType {
    match inst_type {
        "t2.micro" => InstanceType::T2Micro,
        "t3.small" => InstanceType::T3Small,
        _ => InstanceType::T2Nano,
    }
}

/// Perform actions in aws based on the symbol table
pub(crate) async fn apply_aws(aws_sym: &AwsSym) -> Result<(), AwsDeployError> {
    let region_provider = RegionProviderChain::default_provider().or_else(AWS_REGION);
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let mut errors: Vec<AwsDeployError> = vec![];
    for ec2 in &aws_sym.ec2s {
        let inst_type = get_instance_type(&ec2.instance_type);
        println!(
            "ec2: {}, ami: {}, type: {}",
            ec2.name, ec2.ami_id, inst_type
        );
        let ec2_inst = actions::instances::create_instance(
            &config,
            ec2.name.as_str(),
            ec2.ami_id.as_str(),
            ec2.count.into(),
            inst_type,
            ec2.subnet_id.as_str(),
            ec2.key_name.as_str(),
            vec![ec2.sg_id.as_str()],
        )
        .await;

        match ec2_inst {
            Ok(_) => {
                //
            }
            Err(e) => {
                errors.push(AwsDeployError::new(AwsErrorType::EC2Deploy, e.to_string()));
            }
        }
    }
    if !errors.is_empty() {
        let mut s = String::from("");
        for err in &errors {
            s = format!("{}\n-> {}", s, err);
        }
        return Err(AwsDeployError::new(AwsErrorType::EC2Deploy, s));
    }

    Ok(())
}
