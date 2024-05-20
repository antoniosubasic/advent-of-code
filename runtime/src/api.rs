use anyhow::{Error, Result};
use regex::Regex;
use reqwest::Client;
use std::{collections::HashMap, path::PathBuf};

use crate::PuzzleData;

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
    ) -> Result<String, Error> {
        let client = Client::new();
        let mut request = client.request(method, uri);

        request = request.header("Cookie", format!("session={}", self.cookie));
        if let Some(headers) = headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        if let Some(content) = content {
            request = request.body(content);
        }

        Ok(request.send().await?.text().await?)
    }

    pub async fn get_input_text(&self, year: u16, day: u8) -> Result<String> {
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

    pub async fn submit_answer(&self, year: u16, day: u8, part: u8, answer: &str) -> Result<bool> {
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
                .ok_or(Error::msg("time could not be found"))?;

            Err(Error::msg(format!(
                "cooldown left: {}",
                &regex_match["time"]
            )))
        } else {
            Ok(false)
        }
    }

    pub async fn ensure_input(&self, input_file: &PathBuf, puzzle_data: &PuzzleData) -> Result<()> {
        if !input_file.exists() {
            let parent_directory = input_file
                .parent()
                .ok_or(Error::msg("could not find parent"))?;

            if !parent_directory.exists() {
                std::fs::create_dir_all(parent_directory)?;
            }

            let input = self
                .get_input_text(puzzle_data.year, puzzle_data.day)
                .await?;

            std::fs::write(input_file, input)?;
        }

        Ok(())
    }

    pub async fn evaluate_solution(
        &self,
        solution_file: &PathBuf,
        year: u16,
        day: u8,
        solution: (&str, &str),
    ) -> Result<(bool, bool)> {
        Ok(if solution_file.exists() {
            let solution_content = std::fs::read_to_string(solution_file)?;

            let mut lines = solution_content.lines();
            let part1 = lines.next().ok_or(Error::msg("part 1 not found"))?;
            let part2 = lines.next().ok_or(Error::msg("part 2 not found"))?;

            (part1 == solution.0, part2 == solution.1)
        } else {
            let part1_result = self.submit_answer(year, day, 1, solution.0).await?;
            let part2_result = self.submit_answer(year, day, 2, solution.1).await?;

            if part1_result && part2_result {
                std::fs::write(solution_file, format!("{}\n{}", solution.0, solution.1))?;
            }

            (part1_result, part2_result)
        })
    }
}
