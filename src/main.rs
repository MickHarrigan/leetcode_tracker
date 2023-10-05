mod commands;
use commands::{
    common::*, edit::*, finish::*, hide::*, info::*, interpret::*, new::*, search::*, submit::*,
    tag::*,
};

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    match &args.command {
        Commands::New { link } => {
            commands::new::run(link).await? // <--- Remove this ; later as it should return all the way up to
                                            // main
        }

        Commands::Edit { num } => {
            // takes a number then allows the user to edit the solution
            // this is the mutating version of Commands::Info.
            commands::edit::run(num)?
        }

        Commands::Tag { cmd } => tag_subcommands(cmd)?,
        Commands::Info { num } => {
            // takes a number and prints a bunch of info about the problem
        }
        Commands::Search { cmd } => {
            // given any of (name, number, tag(s)) will find what you are searching for
        }
        Commands::Hide { num } => {
            // given a number will tag this as a hidden problem that has been attempted but not
            // completed. This should maybe be pushed somewhere else or just not tracked.
        }
        Commands::Test { num } => {
            // same as submit but with the altered state of running the tests that LC provides
        }
        Commands::Submit { num } => {
            // given a number should aim to send the code to LeetCode, but I have no idea on how
            // to actually send this to them and receive the response. HTTP? GraphQL? I have no
            // clue.
        }
        Commands::Finish { num } => {
            // Ceremoniously tags the problem as completed and with whichever solution was used.
            // Maybe this can track multiple solutions as well to be able to compare them.
            //
            // Hopefully this can also have a way to see the time/space complexities and
            // explanations of functions but we will see.
        }
        #[allow(unreachable_patterns)]
        a => println!("Input was: {a:?}"),
    }
    Ok(())
}

#[cfg(test)]
mod test;
