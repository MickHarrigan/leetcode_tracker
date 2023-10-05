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
        /**********************************************************************/
        // These are the commands that are most necessary
        Commands::New { link } => commands::new::run(link).await?,

        Commands::Edit { num } => commands::edit::run(num)?,

        Commands::Tag { cmd } => tag_subcommands(cmd)?,

        Commands::Test { num } => {
            // same as submit but with the altered state of running the tests that LC provides
        }
        Commands::Submit { num } => {
            // given a number should aim to send the code to LeetCode, but I have no idea on how
            // to actually send this to them and receive the response. HTTP? GraphQL? I have no
            // clue.
        }
        /**********************************************************************/
        // these are the commands that can be remade for other uses
        Commands::Search { cmd } => {
            // given any of (name, number, tag(s)) will find what you are searching for
        }
        Commands::Info { num } => {
            // takes a number and prints a bunch of info about the problem
        }
        Commands::Hide { num } => {
            // given a number will tag this as a hidden problem that has been attempted but not
            // completed. This should maybe be pushed somewhere else or just not tracked.
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
