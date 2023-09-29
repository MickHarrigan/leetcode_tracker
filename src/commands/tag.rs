use anyhow::Result;
use clap::{Parser, ValueEnum};
use regex::Regex;
use std::{
    fs::File,
    io::{BufReader, Write},
};
use strum::{EnumString, IntoStaticStr};

use super::common::{get_lc_dir, prompt_for_input};

#[derive(Debug, Clone, Parser, EnumString, IntoStaticStr)]
pub enum TagType {
    // #[strum(serialize = "stack", serialize = "<other name>")]
    #[strum(serialize = "stack", ascii_case_insensitive)]
    Stack,

    #[strum(serialize = "dp", ascii_case_insensitive)]
    DynamicProgramming,

    #[strum(serialize = "btree", ascii_case_insensitive)]
    BinaryTree,

    #[strum(serialize = "graph", ascii_case_insensitive)]
    Graph,

    #[strum(serialize = "backtracking", ascii_case_insensitive)]
    BackTracking,
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

impl std::fmt::Display for TagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn tag_subcommands(cmd: &TagCommand) -> Result<()> {
    match cmd {
        TagCommand::Add => {
            // FLAGS
            // ************************************************************
            // Add should take a number and a tag/[tags]
            // to apply to a problem referenced by the number provided
            //
            // PROMPTS
            // ************************************************************
            // should prompt for a tag and a problem number
            let lc_dir = get_lc_dir()?;
            let (_input_tag, tag) = prompt_for_input::<TagType>("Enter Tag to add: ")?;

            let (_input_num, num) =
                prompt_for_input::<usize>("Enter Problem Number to add Tag to: ")?;

            let problem_dir = format!("{}{}{}", lc_dir, "src/", num);
            let tag_path = format!("{}{}", problem_dir, "/TAGS");
            let tag_file = std::fs::read_to_string(tag_path.clone())?;

            // search for that tag in the problem, if it already exists, return an error of that
            let re = Regex::new(format!(r"{}", tag).as_str()).unwrap();
            if let Some(_a) = re.captures(tag_file.as_str()) {
                return Err(anyhow::Error::msg(format!(
                    "Tag `{tag}` already exists for this problem!"
                )));
            } else {
                let mut file = std::fs::OpenOptions::new().append(true).open(tag_path)?;
                writeln!(file, "{}", tag.to_string())?;
            }

            println!("Tag: {tag:?} was added to Problem: {num}");
            Ok(())
        }
        TagCommand::Remove => {
            // FLAGS
            // ************************************************************
            // should take a number and a tag/[tags] to remove the tags from said problem
            //
            // PROMPTS
            // ************************************************************
            // should prompt for a tag and a problem number

            let lc_dir = get_lc_dir()?;

            let (_input_num, num) =
                prompt_for_input::<usize>("Enter Problem Number to remove a Tag from: ")?;

            let prompt = format!("Enter Tag to remove from Problem {}: ", num);
            let (_input_tag, tag) = prompt_for_input::<TagType>(prompt.as_str())?;

            let problem_dir = format!("{}{}{}", lc_dir, "src/", num);
            let tag_path = format!("{}{}", problem_dir, "/TAGS");
            let tag_file = std::fs::read_to_string(tag_path.clone())?;

            // file contents without the tag that is to be removed
            let mut buf = Vec::new();
            for line in tag_file.lines() {
                if line.to_string() != tag.to_string() {
                    buf.push(line);
                }
            }
            let buf = buf.join("\n");

            let re = Regex::new(format!(r"{}", tag).as_str()).unwrap();

            if let Some(_a) = re.captures(tag_file.as_str()) {
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(tag_path)?;
                writeln!(file, "{}", buf)?;
            } else {
                return Err(anyhow::Error::msg(format!(
                    "Tag `{tag}` doesn't exist for this problem!"
                )));
            }
            println!("Tag: {tag:?} was removed from Problem: {num}");
            Ok(())
        }
        TagCommand::Edit => {
            // should just take a number and then give a list of all tags for that problem that
            // the user can adjust
            todo!();
        }
        TagCommand::Search => {
            // given a tag, finds all problems that have that tag
            todo!();
        }
    }
}
