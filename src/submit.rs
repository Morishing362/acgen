use super::session::Session;
use super::utils;

use regex::Regex;
use std::fs::File;
use std::io::Read;

pub async fn submit(session: &mut Box<Session>) -> Result<(), Box<dyn std::error::Error>> {
    if !session.has_login_cache().await? {
        session.login().await?;
    }

    let (contest_id, problem_id) = utils::get_task_info();

    // Make submit url.
    let url = format!(r#"https://atcoder.jp/contests/{}/submit"#, contest_id,);

    // Get CSRF token.
    let res = session.get_request(url.as_str()).await?;
    let body = res.text().await?.clone();
    let csrf_token = utils::get_csrf_token_from(body);

    // Read main.rs file.
    let mut file = File::open("src/main.rs").expect("Failed to open `src/main.rs`.");
    let mut main_file_body = String::new();
    file.read_to_string(&mut main_file_body)
        .expect("Fialed to read `src/main.rs`.");

    // Capture main function.
    let regex = Regex::new(r#"([\s\S]*)Copy Above!\n"#).unwrap();
    let capture = regex
        .captures(&main_file_body)
        .expect("`Copy Above!` section not found");
    let source = capture.get(0).expect("main function not found.").as_str();

    let form_data = [
        ("csrf_token", csrf_token.as_str()),
        ("data.TaskScreenName", problem_id.as_str()),
        ("data.LanguageId", "4050"),
        ("sourceCode", source),
    ];

    // Submit.
    let res = session.post_request(url.as_str(), &form_data).await?;

    println!("Submit: {}", res.status().to_string());

    Ok(())
}
