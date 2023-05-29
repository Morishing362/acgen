use regex::Regex;
use std::{fs::File, io::Read};

pub fn get_csrf_token_from(html_body: String) -> String {
    let csrf_input_tag = Regex::new(r#"<input type="hidden" name="csrf_token" .* />"#)
        .unwrap()
        .captures(&html_body)
        .expect("csrf_token input not found")
        .get(0)
        .expect("csrf_token not found.")
        .as_str();

    let csrf_value = Regex::new(r#"value=".*""#)
        .unwrap()
        .captures(&csrf_input_tag)
        .expect("csrf_token input not found")
        .get(0)
        .expect("csrf_token not found.")
        .as_str();

    String::from(&csrf_value[7..csrf_value.len() - 1])
}

pub fn get_user_info_from_std() -> (String, String) {
    let (mut username_buff, mut password_buff) = (String::new(), String::new());

    println!("Enter username:");
    std::io::stdin()
        .read_line(&mut username_buff)
        .expect("Failed to read line");

    println!("Enter password:");
    std::io::stdin()
        .read_line(&mut password_buff)
        .expect("Failed to read line");

    (
        username_buff.trim().to_string(),
        password_buff.trim().to_string(),
    )
}

/// Returns (contest_id, problem_id).
pub fn get_task_info() -> (String, String) {
    // Read toml file.
    let mut toml_body = String::new();
    File::open("Cargo.toml")
        .expect("Failed to open `Cargo.toml`.")
        .read_to_string(&mut toml_body)
        .expect("Failed to read `Cargo.toml`.");

    // Get problem ID and contest ID from toml body.
    let problem_id = Regex::new(r#"abc[0-9]{3}_[a-zA-Z]{1,3}"#)
        .expect("Problem id not found.")
        .captures(&toml_body)
        .unwrap()
        .get(0)
        .expect("Problem id not found.")
        .as_str();

    (String::from(&problem_id[0..6]), String::from(problem_id))
}

/// Returns a file body as String.
pub fn read_template_file(template: &str) -> String {
    match File::open(template) {
        Ok(mut file) => {
            let mut header = String::new();
            file.read_to_string(&mut header)
                .expect("Failed to read file to String.");
            header
        }
        Err(_) => {
            println!("header.txt not found.");
            String::new()
        }
    }
}
