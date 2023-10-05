use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Parser)]
pub enum SearchCommand {
    Number,
    Name,
    Tag,
}

impl ValueEnum for SearchCommand {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Tag => clap::builder::PossibleValue::new("tag"),
            Self::Name => clap::builder::PossibleValue::new("name"),
            Self::Number => clap::builder::PossibleValue::new("number"),
        })
    }
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Name, Self::Number, Self::Tag]
    }
}
