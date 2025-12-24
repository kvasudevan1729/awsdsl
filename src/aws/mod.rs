use std::fmt;

pub(crate) mod nodes;
pub(crate) mod parser;

pub(crate) trait ParseTree {
    fn print_ast(&self, n_spaces: u8) -> String;
}

pub(crate) struct AwsSym {
    id: String,
    pub(crate) region: String,
    pub(crate) ec2s: Vec<Ec2Sym>,
}

impl AwsSym {
    pub(crate) fn new(id: String, region: String) -> Self {
        AwsSym {
            id: id,
            region: region,
            ec2s: vec![],
        }
    }

    pub(crate) fn add_ec2(&mut self, ec2: Ec2Sym) {
        self.ec2s.push(ec2);
    }
}

impl fmt::Display for AwsSym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ec2_s = String::from("");
        for ec2 in &self.ec2s {
            ec2_s = format!("{}\n {}", ec2_s, ec2)
        }
        write!(f, "[{}], region: {}{}", self.id, self.region, ec2_s)
    }
}
pub(crate) struct Ec2Sym {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) desc: String,
    pub(crate) instance_type: String,
    pub(crate) ami_id: String,
    pub(crate) subnet_id: String,
    pub(crate) sg_id: String,
    pub(crate) app_version: f32,
    pub(crate) count: u8,
}

impl fmt::Display for Ec2Sym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}], name: {}", self.id, self.name)
    }
}

impl Ec2Sym {
    pub(crate) fn new(
        name: String,
        desc: String,
        instance_type: String,
        ami_id: String,
        subnet_id: String,
        sg_id: String,
        app_version: f32,
        count: u8,
    ) -> Self {
        Ec2Sym {
            id: (&name).to_string(),
            name: name,
            desc: desc,
            instance_type: instance_type,
            ami_id: ami_id,
            subnet_id: subnet_id,
            sg_id: sg_id,
            app_version: app_version,
            count: count,
        }
    }
}
