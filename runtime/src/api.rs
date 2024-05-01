use regex::Regex;
use reqwest::Client;
use std::collections::HashMap;
use std::error::Error;

pub struct Session {
    cookie: String,
}

impl Session {
    pub fn new(cookie: String) -> Self {
        Self {
            cookie: cookie.trim_end_matches('\n').to_string(),
        }
    }

    async fn send_request(
        &self,
        method: reqwest::Method,
        uri: &str,
        headers: Option<HashMap<&str, &str>>,
        content: Option<reqwest::Body>,
    ) -> Result<String, reqwest::Error> {
        let client = Client::new();
        let mut request = client.request(method, uri);

        request = request.header(
            "Cookie",
            format!("session={}", self.cookie),
        );
        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        if let Some(content) = content {
            request = request.body(content);
        }

        let response = request.send().await?;
        response.text().await
    }

    pub async fn get_input_text(&self, year: u16, day: u8) -> Result<String, Box<dyn Error>> {
        let response = self
            .send_request(
                reqwest::Method::GET,
                &format!("https://adventofcode.com/{}/day/{}/input", year, day),
                None,
                None,
            )
            .await?;

        Ok(response.trim_end().to_string())
    }

    pub async fn submit_answer(
        &self,
        year: u16,
        day: u8,
        part: u8,
        answer: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let response = self
            .send_request(
                reqwest::Method::POST,
                &format!("https://adventofcode.com/{}/day/{}/answer", year, day),
                Some(
                    [("Content-Type", "application/x-www-form-urlencoded")]
                        .iter()
                        .cloned()
                        .collect(),
                ),
                Some(format!("level={}&answer={}", part, answer).into()),
            )
            .await?;

        if response.contains("That's the right answer!") {
            Ok(true)
        } else if response
            .contains("You don't seem to be solving the right level.  Did you already complete it?")
        {
            let day_response = self
                .send_request(
                    reqwest::Method::GET,
                    &format!("https://adventofcode.com/{}/day/{}", year, day),
                    None,
                    None,
                )
                .await?;

            let puzzle_answer_regex =
                Regex::new(r#"<p>Your puzzle answer was <code>(?<answer>.*?)</code>.</p>"#)?;

            let mut match_count = 0;

            for capture in puzzle_answer_regex.captures_iter(&day_response) {
                if match_count == part - 1 {
                    if let Some(answer_found) = capture.name("answer") {
                        if answer_found.as_str() == answer {
                            return Ok(true);
                        }
                    }
                }

                match_count += 1;
            }

            Ok(false)
        } else if response.contains("You gave an answer too recently") {
            let answer_too_recent_regex = Regex::new(r#"You have (?P<time>.*?) left to wait"#)?;
            let regex_match = answer_too_recent_regex
                .captures(&response)
                .ok_or("time could not be found")?;
            Err(format!("cooldown left: {}", &regex_match["time"]).into())
        } else {
            Ok(false)
        }
    }
}
