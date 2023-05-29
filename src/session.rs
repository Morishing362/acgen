use crate::utils;

use reqwest::{self, cookie::Cookie, Response};
use std::{
    fs::File,
    io::{Read, Write},
};

pub struct Session {
    client: Box<reqwest::Client>,
    cookie_header: String,
}

impl Session {
    pub fn new() -> Session {
        Session {
            client: Box::new(
                reqwest::Client::builder()
                    .redirect(reqwest::redirect::Policy::none())
                    .build()
                    .unwrap(),
            ),
            cookie_header: String::new(),
        }
    }

    pub async fn has_cookies(
        &mut self,
        work_space: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // Read toml file.
        match File::open(format!("{}/cookies.txt", work_space)) {
            Ok(mut file) => {
                let mut cookie_header = String::new();
                file.read_to_string(&mut cookie_header)?;
                println!("The login session exists.");

                // Update cookies.
                self.cookie_header = cookie_header;
                Ok(true)
            }
            Err(_) => {
                println!("No login session.");
                Ok(false)
            }
        }
    }

    pub async fn login(&mut self, work_space: String) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.get_request("https://atcoder.jp/login").await?;

        // Get cookies of the login session.
        self.cookie_header = res
            .cookies()
            .collect::<Vec<Cookie>>()
            .into_iter()
            .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
            .collect::<Vec<String>>()
            .join("; ");

        // Get CSRF token.
        let csrf_token = utils::get_csrf_token_from(res.text().await?);

        // Get username & password.
        let (username, password) = utils::get_user_info_from_std();

        // Login request.
        let form_data = [
            ("username", username.as_str()),
            ("password", password.as_str()),
            ("csrf_token", csrf_token.as_str()),
        ];

        let res = self
            .post_request("https://atcoder.jp/login", &form_data)
            .await?;

        println!("Login: {}", res.status().to_string());

        // Cache cookies.
        let cookie_header = res
            .cookies()
            .collect::<Vec<Cookie>>()
            .into_iter()
            .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
            .collect::<Vec<String>>()
            .join("; ");

        let mut file = File::create(format!("{}/cookies.txt", work_space))?;
        file.write_all(cookie_header.as_bytes())?;

        // Update cookies.
        self.cookie_header = cookie_header;

        Ok(())
    }

    pub async fn get_request(&self, url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        Ok(self
            .client
            .get(url)
            .header(
                reqwest::header::COOKIE,
                reqwest::header::HeaderValue::from_str(&self.cookie_header.as_str())?,
            )
            .send()
            .await?)
    }

    pub async fn post_request(
        &self,
        url: &str,
        form_data: &[(&str, &str)],
    ) -> Result<Response, Box<dyn std::error::Error>> {
        Ok(self
            .client
            .post(url)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .header(
                reqwest::header::COOKIE,
                reqwest::header::HeaderValue::from_str(self.cookie_header.as_str())?,
            )
            .form(form_data)
            .send()
            .await?)
    }
}
