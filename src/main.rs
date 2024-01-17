mod aws;
mod config;
mod error;
mod ip;
mod user;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let current_ipv4 = ip::get_public_ip().await?;

    // TODO: handle root user and non-existing home directory
    let user_home_dir = user::get_home_dir_of_current_user().unwrap();

    match config::load_config(user_home_dir.as_str()) {
        Ok(config) => match config {
            Some(config) => {
                aws::update_aws_sg_inbound_rules(&config, &current_ipv4).await;
            }
            None => {
                println!("Loaded a default configuration. Please configure it.");
                std::process::exit(0);
            }
        },
        Err(error) => {
            eprintln!("Error loading configuration: {:?}", error);
            std::process::exit(error::ErrorCode::ConfigReadError as i32);
        }
    }
    Ok(())
}
