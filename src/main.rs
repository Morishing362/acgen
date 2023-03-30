#![allow(unused)]

use regex::Regex;
use reqwest;
use std::{
    collections::VecDeque,
    env,
    fs::{self, File},
    io::Write,
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: acgen <url>");
        return Ok(());
    }

    let url = String::from(&args[1]);
    let client = reqwest::Client::new();

    let res = client.get(url).send().await?;

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
    let project_name = "solution";
    fs::create_dir(project_name).unwrap();

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

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
cli_test_dir = "*"
"#,
        project_name
    );

    file.write_all(dependencies.as_bytes()).unwrap();

    // Create main.rs file
    let main_file = format!("{}/main.rs", src_dir);
    let mut file = File::create(&main_file).unwrap();

    let mut main = format!(
        r#"#![allow(unused)]

use core::convert::TryInto;
use std::{{
    collections::{{HashMap, HashSet, VecDeque}},
    io,
    iter::FromIterator,
}};

macro_rules! read_values {{
    ( $t:ty; $( $x:ident ),+ ) => {{
        let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
        let mut splited_line = line.trim().split(' ').flat_map(str::parse::<$t>);
        $(
            let $x: $t = splited_line.next().expect("Failed to parse input");
        )*
    }};
}}

macro_rules! read_vector {{
    ( $t:ty; $v:ident ) => {{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let $v: Vec<$t> = line.trim().split(' ').flat_map(str::parse::<$t>).collect();
    }};
}}

macro_rules! read_matrix {{
    ( $t:ty; $n:expr; $m:expr; $v:ident ) => {{
        let mut matrix: Vec<Vec<$t>> = Vec::new();
        for _i in 0..$n {{
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            let splited_line: Vec<$t> = line.trim().split(' ').flat_map(str::parse::<$t>).collect();
            assert_eq!(splited_line.len(), $m);
            matrix.push(splited_line);
        }}
        let $v = matrix;
    }};
}}

macro_rules! print_vectorln {{
    ( $v:ident ) => {{
        println!(
            "{{}}",
            $v.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }};
}}

fn main() {{
    // Code Here!
}}

// Copy Above!
mod tests {{
    use cli_test_dir::*;

    const BIN: &'static str = "solution";

"#
    );

    let hashtag = "#";
    for (i, (input, output)) in sample_inputs.iter().zip(sample_outputs.iter()).enumerate() {
        let input = input.to_string() + &String::from("\n");
        let output = output.to_string() + &String::from("\n");
        let content = format!(
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
        main += &content;
    }

    main += "}\n";
    file.write_all(main.as_bytes()).unwrap();

    Ok(())
}
