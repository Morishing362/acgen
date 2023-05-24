use super::session::Session;
use super::utils;

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

    // TODO read main.rs file.
    let body = "Hello, World!";

    let form_data = [
        ("csrf_token", csrf_token.as_str()),
        ("data.TaskScreenName", problem_id.as_str()),
        ("data.LanguageId", "4050"),
        ("sourceCode", body),
    ];

    // Submit.
    let res = session.post_request(url.as_str(), &form_data).await?;

    println!("Submit: {}", res.status().to_string());

    Ok(())
}
