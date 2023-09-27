use clap::{Parser, ValueEnum};
use strum::{EnumString, IntoStaticStr};

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
