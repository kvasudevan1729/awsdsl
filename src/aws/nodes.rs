use std::fmt;

use crate::aws::ParseTree;

pub(crate) struct AwsNode {
    pub(crate) id: String,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) region: Option<String>,
    pub(crate) ec2_nodes: Vec<Ec2Node>,
}

impl AwsNode {
    pub(crate) fn new(id: String) -> Self {
        AwsNode {
            id: id,
            ec2_nodes: vec![],
            region: None,
            name: None,
            description: None,
        }
    }

    pub(crate) fn set_name(&mut self, name: impl std::convert::Into<String>) {
        self.name = Some(name.into());
    }

    pub(crate) fn set_description(&mut self, description: impl std::convert::Into<String>) {
        self.description = Some(description.into());
    }

    pub(crate) fn set_region(&mut self, region: impl std::convert::Into<String>) {
        self.region = Some(region.into());
    }

    pub(crate) fn add_ec2(&mut self, ec2: Ec2Node) {
        self.ec2_nodes.push(ec2);
    }
}

impl fmt::Display for AwsNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ec2_s = "".to_string();
        for ec2 in &self.ec2_nodes {
            ec2_s = format!("{}\n -- {}", ec2_s, ec2)
        }
        write!(
            f,
            "---\n{}[{}]{}",
            self.id,
            self.region.as_ref().unwrap(),
            ec2_s
        )
    }
}

impl ParseTree for AwsNode {
    fn print_ast(&self, n_spaces: u8) -> String {
        let mut s = String::from("[aws]");
        let empty_spaces = " ".repeat(n_spaces as usize);
        s = format!(
            "{}\n{}- [name]: {}",
            s,
            empty_spaces,
            self.name.as_ref().unwrap()
        );
        s = format!(
            "{}\n{}- [description]: {}",
            s,
            empty_spaces,
            self.description.as_ref().unwrap()
        );
        s = format!(
            "{}\n{}- [region]: {}",
            s,
            empty_spaces,
            self.region.as_ref().unwrap()
        );
        for ec2 in &self.ec2_nodes {
            s = format!("{}\n{}", s, ec2.print_ast(n_spaces));
        }

        s
    }
}

pub(crate) struct Ec2Node {
    pub(crate) id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) instance_type: Option<String>,
    pub(crate) count: u8,
    pub(crate) app_version: f32,
    pub(crate) ami: Option<String>,
    pub(crate) subnet_id: Option<String>,
    pub(crate) sg_id: Option<String>,
}

impl Ec2Node {
    pub(crate) fn new() -> Self {
        Ec2Node {
            id: None,
            name: None,
            description: None,
            instance_type: None,
            count: 0,
            ami: None,
            app_version: 0.0,
            subnet_id: None,
            sg_id: None,
        }
    }

    pub(crate) fn set_name(&mut self, name: impl std::convert::Into<String>) {
        let s = name.into();
        self.id = Some(s.clone());
        self.name = Some(s);
    }

    pub(crate) fn set_description(&mut self, description: impl std::convert::Into<String>) {
        self.description = Some(description.into());
    }

    pub(crate) fn set_instance_type(&mut self, instance_type: impl std::convert::Into<String>) {
        self.instance_type = Some(instance_type.into());
    }

    pub(crate) fn set_ami(&mut self, ami: impl std::convert::Into<String>) {
        self.ami = Some(ami.into());
    }

    pub(crate) fn set_subnet_id(&mut self, subnet_id: impl std::convert::Into<String>) {
        self.subnet_id = Some(subnet_id.into());
    }

    pub(crate) fn set_sg_id(&mut self, sg_id: impl std::convert::Into<String>) {
        self.sg_id = Some(sg_id.into());
    }

    pub(crate) fn set_count(&mut self, count: u8) {
        self.count = count;
    }

    pub(crate) fn set_app_version(&mut self, app_version: f32) {
        self.app_version = app_version;
    }
}

impl fmt::Display for Ec2Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}[{}, x {}]",
            self.name.as_ref().unwrap(),
            self.instance_type.as_ref().unwrap(),
            self.count
        )
    }
}

impl ParseTree for Ec2Node {
    fn print_ast(&self, n_spaces: u8) -> String {
        let mut empty_spaces = " ".repeat(n_spaces as usize);
        let mut s = format!("{}- [ec2]", empty_spaces);
        empty_spaces = " ".repeat((n_spaces * 2) as usize);
        s = format!(
            "{}\n{}- [name]: {}",
            s,
            empty_spaces,
            self.name.as_ref().unwrap()
        );
        s = format!(
            "{}\n{}- [description]: {}",
            s,
            empty_spaces,
            self.description.as_ref().unwrap()
        );
        s = format!(
            "{}\n{}- [instance_type]: {}",
            s,
            empty_spaces,
            self.instance_type.as_ref().unwrap()
        );
        s = format!("{}\n{}- [count]: {}", s, empty_spaces, self.count);
        s = format!(
            "{}\n{}- [app_version]: {}",
            s, empty_spaces, self.app_version
        );
        s = format!(
            "{}\n{}- [ami]: {}",
            s,
            empty_spaces,
            self.ami.as_ref().unwrap()
        );
        s = format!(
            "{}\n{}- [subnet_id]: {}",
            s,
            empty_spaces,
            self.subnet_id.as_ref().unwrap()
        );
        s = format!(
            "{}\n{}- [sg_id]: {}",
            s,
            empty_spaces,
            self.sg_id.as_ref().unwrap()
        );

        s
    }
}
