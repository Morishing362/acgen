use regex::Regex;
use std::{fs::File, io::Read};

pub fn get_csrf_token_from(html_body: String) -> String {
    let csrf_input_regex = Regex::new(r#"<input type="hidden" name="csrf_token" .* />"#).unwrap();
    let capture = csrf_input_regex
        .captures(&html_body)
        .expect("csrf_token input not found");
    let csrf_input_tag = capture.get(0).expect("csrf_token not found.").as_str();

    let csrf_value_regex = Regex::new(r#"value=".*""#).unwrap();
    let capture = csrf_value_regex
        .captures(&csrf_input_tag)
        .expect("csrf_token input not found");
    let csrf_value = capture.get(0).expect("csrf_token not found.").as_str();

    let csrf_token = &csrf_value[7..csrf_value.len() - 1];

    String::from(csrf_token)
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

    let username = username_buff.trim();
    let password = password_buff.trim();

    (String::from(username), String::from(password))
}

pub fn get_task_info() -> (String, String) {
    // Read toml file.
    let mut file = File::open("Cargo.toml").expect("Failed to open `Cargo.toml`.");
    let mut toml_body = String::new();
    file.read_to_string(&mut toml_body)
        .expect("Failed to read `Cargo.toml`.");

    // Get problem ID and contest ID from toml body.
    let problem_id_regex =
        Regex::new(r#"abc[0-9]{3}_[a-zA-Z]{1,3}"#).expect("Problem id not found.");
    let capture = problem_id_regex.captures(&toml_body).unwrap();
    let problem_id = capture.get(0).expect("Problem id not found.").as_str();
    let contest_id = &problem_id[0..6];

    (String::from(contest_id), String::from(problem_id))
}
