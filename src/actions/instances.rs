use aws_config::SdkConfig;
use aws_sdk_ec2::client::Waiters;
use aws_sdk_ec2::types::{InstanceType, Tag};
use aws_sdk_ec2::{Client, Error};
use std::time::Duration;

pub(crate) async fn show_instances(config: &SdkConfig) -> Result<(), Error> {
    let client = Client::new(&config);
    let instances = client.describe_instances().send().await?;

    for rsrv in instances.reservations() {
        for inst in rsrv.instances() {
            println!(
                "  {}[{}]",
                inst.instance_id().unwrap_or_default(),
                inst.image_id().unwrap_or_default()
            );
        }
    }

    Ok(())
}

pub(crate) async fn create_instance(
    config: &SdkConfig,
    name: impl std::convert::Into<String> + std::fmt::Display,
    ami_id: &str,
    count: i32,
    ec2_size: InstanceType,
    subnet_id: &str,
    key_name: &str,
    sg_ids: Vec<&str>,
) -> Result<(), Error> {
    let client = Client::new(&config);
    let created = client
        .run_instances()
        .image_id(ami_id)
        .instance_type(ec2_size)
        .subnet_id(subnet_id)
        .set_security_group_ids(Some(sg_ids.iter().map(|x| x.to_string()).collect()))
        .set_key_name(Some(key_name.to_string()))
        .min_count(count)
        .max_count(count)
        .send()
        .await?;
    //
    if created.instances().is_empty() {
        println!("No instances created, please check!");
    }

    let mut inst_ids: Vec<String> = vec![];
    match created.instances {
        Some(insts) => {
            for inst in insts {
                let inst_id = inst.instance_id.unwrap_or_default();
                println!("  {}", inst_id);
                inst_ids.push(inst_id.to_string());

                // create tag
                let tag_name = format!("awsdsl - {}", name.to_string());
                let tag_response = client
                    .create_tags()
                    .resources(inst_id)
                    .tags(Tag::builder().key("Name").value(tag_name).build())
                    .send()
                    .await;
                match tag_response {
                    Ok(_) => {
                        println!("Tag set successfully!");
                    }
                    Err(e) => {
                        println!("Error whilst applying tags: {}", e);
                    }
                }
            }
        }
        _ => {
            println!("** No instances created **");
        }
    }

    // check if instance is ready
    let insts_ready = client
        .wait_until_instance_exists()
        .set_instance_ids(Some(inst_ids))
        .wait(Duration::from_secs(5))
        .await;

    match insts_ready {
        Ok(_) => {
            println!("=> instance created!");
        }
        Err(e) => {
            println!("Error: Timed out creating instance!");
            return Err(e.into());
        }
    }
    Ok(())
}
