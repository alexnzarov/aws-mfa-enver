extern crate rusoto_sts;

use rusoto_sts::Credentials;
use std::{io, io::Read, io::Write, process::Command};

pub fn get_string(question: &str) -> String {
    let mut value = String::new();

    println!("{}", question);

    print!("> ");
    io::stdout()
        .flush()
        .expect("Error: Failed to flush stdout.");

    io::stdin()
        .read_line(&mut value)
        .expect("Error: Failed to read input from stdin.");

    String::from(value.trim())
}

pub fn wait_for_input() {
    io::stdin().read(&mut [0u8]).unwrap();
}

fn set_env_variable(key: &str, value: &str) {
    Command::new("setx")
        .arg(key)
        .arg(value)
        .output()
        .expect(&format!("Error: Failed to set {} variable", key));
}

pub fn update_env(credentials: &Credentials) {
    set_env_variable("AWS_ACCESS_KEY_ID", &credentials.access_key_id);
    set_env_variable("AWS_SECRET_ACCESS_KEY", &credentials.secret_access_key);
    set_env_variable("AWS_SESSION_TOKEN", &credentials.session_token);
}
