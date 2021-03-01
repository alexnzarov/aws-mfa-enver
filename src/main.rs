extern crate rusoto_core;
extern crate rusoto_sts;
extern crate tokio;

pub mod config;
pub mod util;

use rusoto_sts::{GetSessionTokenRequest, Sts, StsClient};

#[tokio::main]
async fn main() {
    config::prepare_config();

    let config = config::get_config();
    let client = StsClient::new(config.get_region());

    config.populate_process_env();

    println!("MFA device: {}", config.mfa_serial_number);

    let mfa_code = util::get_string("What is current MFA code?");

    let result = client
        .get_session_token(GetSessionTokenRequest {
            serial_number: Some(config.mfa_serial_number),
            token_code: Some(mfa_code),
            duration_seconds: Some(129600),
        })
        .await;

    match result {
        Ok(response) => match response.credentials {
            Some(credentials) => {
                util::update_env(&credentials);

                println!("Environment variables were updated.");
            }
            None => println!("Error: Failed to fetch credentials."),
        },
        Err(error) => {
            println!("{:?}", error);
        }
    };

    println!("Press enter to exit...");

    util::wait_for_input();
}
