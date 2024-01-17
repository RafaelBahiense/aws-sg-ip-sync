# AWS Security Group IP Sync

## Overview
`aws-sg-ip-sync` is a Rust application designed to automatically update AWS Security Group ingress rules based on the public IP of the machine it's running on. This is especially useful in environments where the IP address changes frequently, ensuring consistent access to AWS resources.

## Releases

This project uses a GitHub Actions workflow to automatically create and release a .deb package each time a new version is pushed. This makes it easy to install and update the application on Debian-based systems.
Using the .deb Release

1. Navigate to the 'Releases' section of the GitHub repository.
2. Download the latest .deb file.
3. Install the package using your preferred Debian package manager, for example, by running sudo dpkg -i filename.deb. (Note: Replace filename.deb with the actual file name of the downloaded release.)

## Prerequisites - Use
To use `aws-sg-ip-sync`, you need:
- An AWS account with permissions to modify Security Group rules.
- `libssl-dev` installed on your machine.
- A Debian-based system. For non-Debian systems and macOS, a pull request is welcome. As for Windows support – I'm still waiting for that window of opportunity to open! 🪟😉

## Configuration
Configure the application by running it the first time, or by creating a `config.toml` file in `~/.config/aws-sg-ip-sync/`. Here's a sample configuration:
```toml
aws_region = "us-east-1"
aws_profile = "default"
aws_sg_id = "sg-123456"
aws_sg_rule_id = "sgr-123456"
```
## Usage
Run the application by executing the compiled binary. The application will:
1. Fetch the public IP of the current machine.
2. Update the specified AWS Security Group ingress rule with the fetched IP.

## Prerequisites - Development
- A Rust environment.

## Building
1. Clone the repository to your local machine.
2. Navigate to the project directory.
3. Build the project using `cargo build --release`.
4. The executable will be available in the `target/release` directory.

## Modules
- **main.rs**: Entry point of the application, orchestrating various module functionalities.
- **aws_module.rs**: Handles AWS interactions, including Security Group rule updates.
- **config_module.rs**: Manages application configuration, including AWS credentials and target descriptions.
- **ip_module.rs**: Fetches the public IP of the current machine.
- **user_module.rs**: Retrieves user-specific information, like the home directory.
- **error.rs**: Defines custom error codes for the application.

## Dependencies
- `reqwest`: For making HTTP requests.
- `aws-sdk-*`: AWS SDK for Rust.
- `serde`: Serialization and deserialization library.
- `toml`: For TOML parsing.
- `tokio`: Asynchronous runtime.
- `users`: For retrieving user information.

## FAQ
- *Why not just a bash script using AWS CLI?* Sorry I don't deal well with logical reasoning, please respect my limitations
- *Do you have any plans to integrate this app with the OpenAI API?* I don't know why, but probably yes


## Contributing
Contributions to the project are welcome. Please follow the standard Git workflow - fork the repository, make changes, and submit a pull request.
