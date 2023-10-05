// common code goes here
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{error::Error, fmt::Debug, io::Write, str::FromStr};

use super::search::SearchCommand;
use super::tag::TagCommand;

pub const SESSION: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJfYXV0aF91c2VyX2lkIjoiMjcyOTYwMyIsIl9hdXRoX3VzZXJfYmFja2VuZCI6ImFsbGF1dGguYWNjb3VudC5hdXRoX2JhY2tlbmRzLkF1dGhlbnRpY2F0aW9uQmFja2VuZCIsIl9hdXRoX3VzZXJfaGFzaCI6IjlkMmI3NjMzMTJiMjAwNjAwNDE1NWM1ODI4NWUzM2M2MTQ2MDJmMzAiLCJpZCI6MjcyOTYwMywiZW1haWwiOiJtaGFycmlnYW4zMjhAZ21haWwuY29tIiwidXNlcm5hbWUiOiJtaGFycmlnMSIsInVzZXJfc2x1ZyI6Im1oYXJyaWcxIiwiYXZhdGFyIjoiaHR0cHM6Ly9zMy11cy13ZXN0LTEuYW1hem9uYXdzLmNvbS9zMy1sYy11cGxvYWQvYXNzZXRzL2RlZmF1bHRfYXZhdGFyLmpwZyIsInJlZnJlc2hlZF9hdCI6MTY5NTY4MTA2MywiaXAiOiI2Ny4xNjQuMTI1LjYwIiwiaWRlbnRpdHkiOiI2ZTUzMWYwNmJmM2ZjMjZmMzZiM2MyODg5NzhlOWZjNCIsInNlc3Npb25faWQiOjQ1OTg0Nzc1LCJfc2Vzc2lvbl9leHBpcnkiOjEyMDk2MDB9.hasC5lHN2_jFX3bUtjbgAjnI9UBGKhXhPxuFLS49fe4";
pub const TOKEN: &str = "aYcsgdAMmffTwhkAEICVGvuj4eR1sZgvrrtCcb2g5LQXPZrXFyXmY7TuDecxYetZ";
pub const GQL_ENDPOINT: &str = "https://leetcode.com/graphql/";
// const COOKIES: &str = format!("LEETCODE_SESSION={};csrftoken={}", SESSION, TOKEN)
// let jar = reqwest::cookie::Jar::default();

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
