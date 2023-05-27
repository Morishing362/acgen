use crate::session::Session;

use regex::Regex;
use std::{
    collections::VecDeque,
    env,
    fs::{self, File},
    io::{Read, Write},
};

pub async fn generate(session: &mut Box<Session>) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let url = String::from(&args[2]);

    if !session.has_login_cache(format!(".")).await? {
        session.login(format!(".")).await?;
    }

    let res = session.get_request(url.as_str()).await?;

    let body = res.text().await?;

    let mut sample_inputs = VecDeque::<String>::new();
    let mut sample_outputs = VecDeque::<String>::new();

    let sample_input_re = Regex::new(r#"(?s)<h3>Sample Input \d+</h3><pre>(.*?)</pre>"#).unwrap();
    for cap in sample_input_re.captures_iter(body.as_str()) {
        let input = cap[1].trim().to_string();
        sample_inputs.push_back(input);
    }

    let sample_output_re = Regex::new(r#"(?s)<h3>Sample Output \d+</h3><pre>(.*?)</pre>"#).unwrap();
    for cap in sample_output_re.captures_iter(body.as_str()) {
        let output = cap[1].trim().to_string();
        sample_outputs.push_back(output);
    }

    assert_eq!(sample_inputs.len(), sample_outputs.len());

    // Create project directory
    let problem_id = url.rsplit('/').next().unwrap();
    let project_name = ["solution_", problem_id].concat();
    fs::create_dir(&project_name).unwrap();

    // Create source directory
    let src_dir = format!("{}/src", project_name);
    fs::create_dir(&src_dir).unwrap();

    // Create Cargo.toml file
    let cargo_file = format!("{}/Cargo.toml", project_name);
    let mut file = File::create(&cargo_file).unwrap();

    let dependencies = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
cli_test_dir = "*"
"#,
        project_name
    );

    file.write_all(dependencies.as_bytes()).unwrap();

    // Create main.rs file
    let main_file = format!("{}/main.rs", src_dir);
    let mut file = File::create(&main_file).unwrap();

    let mut source = String::new();

    let header = match File::open("templates/header.txt") {
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
    };
    source += &header;

    let main = format!(
        r#"
fn main() {{
    // Code Here!
}}

"#
    );
    source += &main;

    let footer = match File::open("templates/footer.txt") {
        Ok(mut file) => {
            let mut footer = String::new();
            file.read_to_string(&mut footer)
                .expect("Failed to read file to String.");
            footer
        }
        Err(_) => {
            println!("footer.txt not found.");
            String::new()
        }
    };
    source += &footer;

    let test_beginning = format!(
        r#"
// Copy Above!
mod tests {{
    use cli_test_dir::*;

    const BIN: &'static str = "solution_{}";

"#,
        problem_id
    );
    source += &test_beginning;

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
    file.write_all(source.as_bytes()).unwrap();

    Ok(())
}
