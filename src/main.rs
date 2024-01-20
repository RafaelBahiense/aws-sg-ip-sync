mod aws;
mod config;
mod error;
mod ip;
mod user;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let current_ipv4 = ip::get_public_ipv4().await?;

    // TODO: handle root user and non-existing home directory
    let user_home_dir = user::get_current_home_dir().expect("Error: user home directory is None");

    match config::load(user_home_dir.as_str()) {
        Ok(config) => {
            if let Some(config) = config {
                aws::update_aws_sg_inbound_rules(&config, &current_ipv4).await;
            } else {
                println!("Loaded a default configuration. Please configure it.");
                std::process::exit(0);
            }
        }

        Err(error) => {
            eprintln!("Error loading configuration: {error:?}");
            std::process::exit(error::Codes::ConfigReadError as i32);
        }
    }
    Ok(())
}
