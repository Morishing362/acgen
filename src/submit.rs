use crate::session::Session;
use crate::utils;

use regex::Regex;
use std::fs::File;
use std::io::Read;

pub async fn submit(session: &mut Box<Session>) -> Result<(), Box<dyn std::error::Error>> {
    if !session.has_cookies(format!("..")).await? {
        session.login(format!("..")).await?;
    }

    let (contest_id, problem_id) = utils::get_task_info();

    // Make submit url.
    let url = format!(r#"https://atcoder.jp/contests/{}/submit"#, contest_id,);

    // Get CSRF token.
    let csrf_token = utils::get_csrf_token_from(
        session
            .get_request(url.as_str())
            .await?
            .text()
            .await?
            .clone(),
    );

    // Read main.rs file.
    let mut main_file_body = String::new();
    File::open("src/main.rs")?
        .read_to_string(&mut main_file_body)
        .expect("Fialed to read `src/main.rs`.");

    // Capture main function.
    let source = Regex::new(r#"([\s\S]*)Copy Above!"#)?
        .captures(&main_file_body)
        .expect("`Copy Above!` section not found")
        .get(0)
        .expect("main function not found.")
        .as_str();

    let form_data = [
        ("csrf_token", csrf_token.as_str()),
        ("data.TaskScreenName", problem_id.as_str()),
        ("data.LanguageId", "4050"),
        ("sourceCode", source),
    ];

    // Submit.
    println!(
        "Submit: {}",
        session
            .post_request(url.as_str(), &form_data)
            .await?
            .status()
            .to_string()
    );

    Ok(())
}
