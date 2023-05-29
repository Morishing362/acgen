use crate::{session::Session, utils::read_template_file};

use regex::Regex;
use std::{
    collections::VecDeque,
    env,
    fs::{self, File},
    io::Write,
};

pub async fn generate(session: &mut Box<Session>) -> Result<(), Box<dyn std::error::Error>> {
    if !session.has_cookies(format!(".")).await? {
        session.login(format!(".")).await?;
    }

    let url = String::from(&env::args().collect::<Vec<String>>()[2]);

    let body = session.get_request(url.as_str()).await?.text().await?;

    let mut sample_inputs = VecDeque::<String>::new();
    let sample_input_re = Regex::new(r#"(?s)<h3>Sample Input \d+</h3><pre>(.*?)</pre>"#)?;
    for cap in sample_input_re.captures_iter(body.as_str()) {
        let input = cap[1].trim().to_string();
        sample_inputs.push_back(input);
    }

    let mut sample_outputs = VecDeque::<String>::new();
    let sample_output_re = Regex::new(r#"(?s)<h3>Sample Output \d+</h3><pre>(.*?)</pre>"#)?;
    for cap in sample_output_re.captures_iter(body.as_str()) {
        let output = cap[1].trim().to_string();
        sample_outputs.push_back(output);
    }

    assert_eq!(sample_inputs.len(), sample_outputs.len());

    // Create project directory
    let problem_id = url
        .rsplit('/')
        .next()
        .expect("Failed to parse the problem id.");
    let project_name = ["solution_", problem_id].concat();
    fs::create_dir(&project_name)?;

    // Create source directory
    let src_dir = format!("{}/src", project_name);
    fs::create_dir(&src_dir)?;

    // Create Cargo.toml file
    let mut file = File::create(&format!("{}/Cargo.toml", project_name))?;

    let dependencies = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
cli_test_dir = "*"
{}
"#,
        project_name,
        read_template_file("templates/dependencies.txt")
    );

    file.write_all(dependencies.as_bytes())?;

    // Create main.rs file
    let mut file = File::create(&format!("{}/main.rs", src_dir))?;

    let mut source = String::new();

    source += &read_template_file("templates/header.txt");

    source += &format!(
        r#"
fn main() {{
    // Code Here!
}}

"#
    );

    source += &read_template_file("templates/footer.txt");

    source += &format!(
        r#"
// Copy Above! (Do not delete this line.)
mod tests {{
    use cli_test_dir::*;

    const BIN: &'static str = "solution_{}";

"#,
        problem_id
    );

    let hashtag = "#";
    for (i, (input, output)) in sample_inputs.iter().zip(sample_outputs.iter()).enumerate() {
        let input = input.to_string() + &String::from("\n");
        let output = output.to_string() + &String::from("\n");
        let tests = format!(
            r#"    #[test]
    fn sample_{}() {{ 
        let testdir = TestDir::new(BIN, "");
        let output = testdir
            .cmd()
            .output_with_stdin(r{}"{}"{},
            )
            .tee_output()
            .expect_success();
        assert_eq!(output.stdout_str(), r{}"{}"{});
        assert!(output.stderr_str().is_empty());
    }}

"#,
            i, hashtag, input, hashtag, hashtag, output, hashtag
        );
        source += &tests;
    }

    source += "}\n";

    file.write_all(source.as_bytes())?;

    Ok(())
}
