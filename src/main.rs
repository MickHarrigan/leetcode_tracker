/* This is a cli tool that creates a new leetcode
 * entry within the `<...>/leetcode repo`.
 *
 * This tool is designed for my own personal use and not really intended for distribution.
 *
 * The goal with this is to make a real cli tool and to cargo install it so
 * I can just use it each day that I make a new LC solution.
 *
 * The longer term goal is to have a way to make this fetch my solutions and
 * information about my submissions, but that is after building this first.
 */

/* Todo:
 * create usage with ./lc (or really lc when its done)
 * should be 3 modes of operation:
 *  1: without args then run through questions to fill in the necessary data
 *  2: with args that fill in the data
 *  3: <Later> a link to the problem that it fills in from
 */

mod args;
mod io;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = args::Args::parse();
    #[cfg(not(feature = "test"))]
    match &args.command {
        args::Commands::New { link } => {
            println!("New command with link: {}", link);
        }
        args::Commands::Tag { cmd } => match cmd {
            args::TagCommand::Add => {}
            args::TagCommand::Edit => {}
            args::TagCommand::Remove => {}
            args::TagCommand::Search => {}
        },
        a => println!("Input was: {a:?}"),
    }
    Ok(())
}
