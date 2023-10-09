// common code goes here
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{error::Error, fmt::Debug, io::Write, str::FromStr};

use super::search::SearchCommand;
use super::tag::TagCommand;

pub const GQL_ENDPOINT: &str = "https://leetcode.com/graphql/";

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
