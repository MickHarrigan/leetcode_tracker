// common code goes here
use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::{error::Error, fmt::Debug, io::Write, str::FromStr};

use crate::tag::TagType;

use super::search::SearchCommand;
use super::tag::TagCommand;

pub const SESSION: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJfYXV0aF91c2VyX2lkIjoiMjcyOTYwMyIsIl9hdXRoX3VzZXJfYmFja2VuZCI6ImFsbGF1dGguYWNjb3VudC5hdXRoX2JhY2tlbmRzLkF1dGhlbnRpY2F0aW9uQmFja2VuZCIsIl9hdXRoX3VzZXJfaGFzaCI6IjlkMmI3NjMzMTJiMjAwNjAwNDE1NWM1ODI4NWUzM2M2MTQ2MDJmMzAiLCJpZCI6MjcyOTYwMywiZW1haWwiOiJtaGFycmlnYW4zMjhAZ21haWwuY29tIiwidXNlcm5hbWUiOiJtaGFycmlnMSIsInVzZXJfc2x1ZyI6Im1oYXJyaWcxIiwiYXZhdGFyIjoiaHR0cHM6Ly9zMy11cy13ZXN0LTEuYW1hem9uYXdzLmNvbS9zMy1sYy11cGxvYWQvYXNzZXRzL2RlZmF1bHRfYXZhdGFyLmpwZyIsInJlZnJlc2hlZF9hdCI6MTY5NTY4MTA2MywiaXAiOiI2Ny4xNjQuMTI1LjYwIiwiaWRlbnRpdHkiOiI2ZTUzMWYwNmJmM2ZjMjZmMzZiM2MyODg5NzhlOWZjNCIsInNlc3Npb25faWQiOjQ1OTg0Nzc1LCJfc2Vzc2lvbl9leHBpcnkiOjEyMDk2MDB9.hasC5lHN2_jFX3bUtjbgAjnI9UBGKhXhPxuFLS49fe4";
pub const TOKEN: &str = "aYcsgdAMmffTwhkAEICVGvuj4eR1sZgvrrtCcb2g5LQXPZrXFyXmY7TuDecxYetZ";
pub const GQL_ENDPOINT: &str = "https://leetcode.com/graphql/";
// const COOKIES: &str = format!("LEETCODE_SESSION={};csrftoken={}", SESSION, TOKEN)
// let jar = reqwest::cookie::Jar::default();

/// Structure containing all the necessary information for a single LeetCode Problem.
/// This includes the title, link, status, difficulty, description, number(id), etc.
#[derive(Debug, Deserialize)]
pub struct LeetCodeProblem {
    pub status: Option<ProblemStatus>,
    pub difficulty: ProblemDifficulty,
    #[serde(rename(deserialize = "frontendQuestionId"))]
    pub frontend_question_id: String,
    pub title: String,
    #[serde(rename(deserialize = "titleSlug"))]
    pub title_slug: String,
    // #[serde(rename(deserialize = "topicTags"))]
    #[serde(skip)]
    pub tags: Vec<TagType>,
    #[serde(skip)]
    pub snippet: String,
    #[serde(skip)]
    pub description: String,
    #[serde(skip)]
    pub tests: String,
    #[serde(rename(deserialize = "acRate"))]
    pub acceptance_rate: f64,
}

#[derive(Debug, Deserialize)]
pub enum ProblemStatus {
    #[serde(rename = "ac")]
    Accepted,
    #[serde(rename = "notac")]
    Attempted,
    NotAttempted,
}
impl fmt::Display for ProblemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ProblemStatus::Accepted => write!(f, "\u{1FBB1}"),
            ProblemStatus::Attempted => write!(f, "\u{1FBC0}"),
            ProblemStatus::NotAttempted => write!(f, "\u{1FBC4}"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum ProblemDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
/// CLI tool to create and manage LeetCode problems
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new problem to be tracked
    #[command(arg_required_else_help = true)]
    New {
        /// A link to the specific LeetCode Problem
        link: String,
    },
    /// Edit a solution for the given problem number
    #[command(arg_required_else_help = true)]
    Edit { num: usize },
    /// Add, Remove, Search, or Edit tags to problems
    #[command(arg_required_else_help = true)]
    Tag { cmd: TagCommand },
    /// Get information about problem status, tags, and solutions
    #[command(arg_required_else_help = true)]
    Info { num: usize },
    /// Hide a question in case you want to come back to it later
    #[command(arg_required_else_help = true)]
    Hide { num: usize },
    /// Search for a problem based on name, tags, or number
    #[command(arg_required_else_help = true)]
    Search { cmd: SearchCommand },
    /// Run the LeetCode provided tests for the provided problem
    #[command(arg_required_else_help = true)]
    Test { num: usize },
    /// Send a solution to be submitted
    #[command(arg_required_else_help = true)]
    Submit { num: usize },
    /// Tag a problem as completed
    #[command(arg_required_else_help = true)]
    Finish { num: usize },
}

pub fn prompt_for_input<T: 'static>(prompt: &str) -> Result<(String, T)>
where
    T: Debug + FromStr,
    <T as FromStr>::Err: Send + Sync + Error,
{
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let mut input = String::from("");
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    let output = input.parse::<T>()?;
    // println!("Input was parsed from: {} into {:?}", input, output);
    Ok((input, output))
}

pub fn get_lc_dir() -> Result<String> {
    use std::env;
    let key = "LEETCODE_DIR";
    // val is the top level directory for the leetcode directory
    env::var(key).map_err(|e| e.into())
}

pub async fn query_endpoint(
    endpoint: String,
    query: serde_json::Value,
    client: reqwest::Client,
) -> Result<serde_json::Value> {
    //     .json(&serde_json::json!({
    //         // replace two-sum with whatever question is in the link
    //         // "query":"\n    query questionTitle($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    title\n    titleSlug\n  }\n}\n    ","variables":{"titleSlug":"two-sum"},"operationName":"questionTitle"
    //         //
    //         // below is how to get function signature and other code
    //         // output["data"]["question"]["codeSnippets"][15]["code"] == code that is
    //         // provided
    //         // "query":"\n    query questionEditorData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    codeSnippets {\n      lang\n      langSlug\n      code\n    }\n    envInfo\n    enableRunCode\n    hasFrontendPreview\n    frontendPreviews\n  }\n}\n    ","variables":{"titleSlug":"two-sum"},"operationName":"questionEditorData"
    //         //
    //         // submissions work
    //         "lang":"rust","question_id":"1","typed_code":"impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n        use std::collections::HashMap;\n        // hash each number with the index as their value\n        let mut hash: HashMap<i32, i32> = HashMap::new();\n        for (k, v) in nums.iter().zip(0..) {\n            match hash.get(&(target - k)) {\n                Some(i) => return vec![v, *i],\n                None => hash.insert(*k, v),\n            };\n        }\n        vec![]\n    }\n}"
    //     }))
    Ok(client
        .post(endpoint)
        .json(&query)
        .send()
        .await?
        .json()
        .await?)
    // Ok(&resp)
}

pub fn generate_request_client(sanitized_link: &Url) -> Result<reqwest::Client> {
    use reqwest::header;
    let cookies = format!("LEETCODE_SESSION={};csrftoken={}", SESSION, TOKEN);

    let mut headers = header::HeaderMap::new();

    let cookie = header::HeaderValue::from_str(cookies.as_str())?;
    let referer = header::HeaderValue::from_str(sanitized_link.as_str())?;
    let csrf = header::HeaderValue::from_str(TOKEN)?;
    let content = header::HeaderValue::from_str("application/json")?;
    let accept = header::HeaderValue::from_str("application/json")?;

    headers.insert(header::COOKIE, cookie);
    headers.insert(header::REFERER, referer);
    headers.insert(header::CONTENT_TYPE, content);
    headers.insert(header::ACCEPT, accept);
    headers.insert(header::HeaderName::from_static("x-csrftoken"), csrf);

    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117")
        .default_headers(headers)
        .cookie_store(true)
        .build()
        .map_err(|e| anyhow::Error::msg(e.to_string()))
}
