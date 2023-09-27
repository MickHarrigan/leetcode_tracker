mod commands;
use commands::{common::*, finish::*, hide::*, info::*, new::*, search::*, submit::*, tag::*};

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    match &args.command {
        Commands::New { link } => {
            let _ = commands::new::run(link).await; // <--- Remove this ; later as it should return all the way up to
                                                    // main
        }

        Commands::Edit { num } => {
            // takes a number then allows the user to edit the solution
            // this is the mutating version of Commands::Info.
        }

        Commands::Tag { cmd } => match cmd {
            TagCommand::Add => {
                // FLAGS
                // ************************************************************
                // Add should take a number and a tag/[tags]
                // to apply to a problem referenced by the number provided
                //
                // PROMPTS
                // ************************************************************
                // should prompt for a tag and a problem number
                let (_input_tag, tag) = prompt_for_input::<TagType>("Enter Tag to add: ")?;

                let (_input_num, num) =
                    prompt_for_input::<usize>("Enter Problem Number to add Tag to: ")?;
                println!("Tag: {tag:?} was added to Problem: {num}");
            }
            TagCommand::Remove => {
                // FLAGS
                // ************************************************************
                // should take a number and a tag/[tags] to remove the tags from said problem
                //
                // PROMPTS
                // ************************************************************
                // should prompt for a tag and a problem number
                let (_input_tag, tag) = prompt_for_input::<TagType>("Enter Tag to add: ")?;

                let (_input_num, num) =
                    prompt_for_input::<usize>("Enter Problem Number to remove Tag from: ")?;
                println!("Tag: {tag:?} was removed from Problem: {num}");
            }
            TagCommand::Edit => {
                // should just take a number and then give a list of all tags for that problem that
                // the user can adjust
            }
            TagCommand::Search => {
                // given a tag, finds all problems that have that tag
            }
        },
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
