use clap::{Parser, Subcommand, ValueEnum};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
#[cfg(not(feature = "test"))]
/// CLI tool to create and manage LeetCode problems
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[cfg(not(feature = "test"))]
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new problem to be tracked
    #[command(arg_required_else_help = true)]
    New {
        /// A link to the specific LeetCode Problem
        link: String,
    },
    /// Add, Remove, Search, or Edit tags to problems
    #[command(arg_required_else_help = true)]
    Tag { cmd: TagCommand },
    /// Get information about problem status, tags, and solutions
    Info,
    /// Hide a question in case you want to come back to it later
    Hide,
    /// Search for a problem based on name, tags, or number
    Search,
    /// Send a solution to be submitted
    Submit,
    /// Tag a problem as completed
    Finish,
}

#[derive(Debug, Clone, Parser)]
pub enum TagCommand {
    /// Add a tag to an existing problem
    Add,
    /// Remove a tag from an existing problem
    Remove,
    /// Edit the tags of a specified problem
    Edit,
    /// Search for all problems with a specific tag
    Search,
}

impl ValueEnum for TagCommand {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Add => clap::builder::PossibleValue::new("add"),
            Self::Remove => clap::builder::PossibleValue::new("remove"),
            Self::Edit => clap::builder::PossibleValue::new("edit"),
            Self::Search => clap::builder::PossibleValue::new("search"),
        })
    }
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Add, Self::Remove, Self::Edit, Self::Search]
    }
}
