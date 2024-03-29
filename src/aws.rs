use crate::config::Config;
use aws_config::meta::region::RegionProviderChain;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_ec2::Client;

pub async fn update_aws_sg_inbound_rules(config: &Config, current_ipv4: &str) {
    let region_provider = RegionProviderChain::first_try(Region::new(config.aws_region.clone()))
        .or_default_provider()
        .or_else(Region::new("us-east-1"));

    let shared_config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .profile_name(&config.aws_profile)
        .load()
        .await;

    let client = Client::new(&shared_config);

    let request = client
        .describe_security_group_rules()
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("group-id")
                .values(&config.aws_sg_id)
                .build(),
        )
        .filters(
            aws_sdk_ec2::types::Filter::builder()
                .name("security-group-rule-id")
                .values(&config.aws_sg_rule_id)
                .build(),
        );

    let response = request.send().await;

    match response {
        Ok(result) => {
            let current_ipv4 = current_ipv4.to_string() + "/32";

            for rule in result
                .security_group_rules
                .expect("No security group rules found")
            {
                let security_group_rule_id = rule
                    .security_group_rule_id
                    .expect("Error: security_group_rule_id is None");
                let cidr_ipv4 = rule.cidr_ipv4.expect("Error: cidr_ipv4 is None");
                let group_id = rule.group_id.expect("Error: group_id is None");

                if security_group_rule_id != config.aws_sg_rule_id {
                    continue;
                }

                println!("Current IPv4: {current_ipv4}");
                println!("Rule ID: {security_group_rule_id}");
                println!("CIDR IPv4: {cidr_ipv4}");

                if cidr_ipv4 == current_ipv4 {
                    println!("CIDR IPv4 is already up to date.");
                    return;
                }

                let response = client
                    .modify_security_group_rules()
                    .group_id(group_id)
                    .security_group_rules(
                        aws_sdk_ec2::types::SecurityGroupRuleUpdate::builder()
                            .security_group_rule_id(security_group_rule_id)
                            .security_group_rule(
                                aws_sdk_ec2::types::SecurityGroupRuleRequest::builder()
                                    .description(rule.description.unwrap_or_default())
                                    .cidr_ipv6(rule.cidr_ipv6.unwrap_or_default())
                                    .cidr_ipv4(current_ipv4.clone())
                                    .from_port(rule.from_port.unwrap_or_default())
                                    .to_port(rule.to_port.unwrap_or_default())
                                    .ip_protocol(rule.ip_protocol.unwrap_or_default())
                                    .prefix_list_id(rule.prefix_list_id.unwrap_or_default())
                                    .build(),
                            )
                            .build(),
                    )
                    .send()
                    .await;

                match response {
                    Ok(_) => {
                        println!("New CIDR IPv4: {current_ipv4}");
                    }
                    Err(e) => {
                        eprintln!("Error: {e:?}");
                    }
                }
            }
        }
        Err(e) => eprintln!("Error: {e:?}"),
    }
}
