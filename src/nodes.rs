use std::fmt;

pub(crate) struct AwsNode {
    id: String,
    name: Option<String>,
    description: Option<String>,
    region: Option<String>,
    ec2_node: Option<Ec2Node>,
}

impl AwsNode {
    pub(crate) fn new(id: String) -> Self {
        AwsNode {
            id: id,
            ec2_node: None,
            region: None,
            name: None,
            description: None,
        }
    }

    pub(crate) fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub(crate) fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub(crate) fn set_region(&mut self, region: String) {
        self.region = Some(region);
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
    name: Option<String>,
    description: Option<String>,
    count: u8,
    app_version: f32,
    image: Option<String>,
    // ec2_attrs: Vec<Ec2AttrNode>,
}

impl Ec2Node {
    pub(crate) fn new(id: String) -> Self {
        Ec2Node {
            id: id,
            name: None,
            description: None,
            count: 0,
            image: None,
            app_version: 0.0,
            // ec2_attrs: vec![],
        }
    }

    pub(crate) fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub(crate) fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub(crate) fn set_image(&mut self, image: String) {
        self.image = Some(image);
    }

    pub(crate) fn set_count(&mut self, count: u8) {
        self.count = count;
    }

    pub(crate) fn set_app_version(&mut self, app_version: f32) {
        self.app_version = app_version;
    }

    // pub(crate) fn add_attr(&mut self, attr: Ec2AttrNode) {
    //     self.ec2_attrs.push(attr);
    // }
}

impl fmt::Display for Ec2Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let mut s = String::new();
        // for attr in &self.ec2_attrs {
        //     s = format!("{}\n{}", s, attr);
        // }
        write!(f, "\n  [{}]", self.id)
    }
}

// pub(crate) struct Ec2AttrNode {
//     id: String,
// }
//
// impl Ec2AttrNode {
//     pub(crate) fn new(id: String) -> Self {
//         Ec2AttrNode { id: id }
//     }
// }
//
// impl fmt::Display for Ec2AttrNode {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "    [{}]", self.id)
//     }
// }
