mod generate;
mod session;
mod submit;
mod utils;

use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: acgen <command> <url>");
        return Ok(());
    }

    let mut session = Box::new(session::Session::new());

    let command = args[1].clone();
    if command == String::from("generate") {
        return generate::generate().await;
    }
    if command == String::from("submit") {
        return submit::submit(&mut session).await;
    }

    println!("Command `{}` was not found.", command);
    Ok(())
}
