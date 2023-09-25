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

use anyhow::Result;
use clap::Parser;
use std::io::Write;
use struct_iterable::Iterable;

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
    // if any of the args are Some(_) then automatically start the
    // non-guided setup
    // use the matching of subcommands
    #[cfg(feature = "test")]
    if args
        .iter()
        .all(|(_name, val)| match val.downcast_ref::<Option<String>>() {
            Some(Some(_)) => true,
            _ => false,
        })
    {
        println!("You included these:");
        for (name, val) in args.iter() {
            if let Some(Some(val)) = val.downcast_ref::<Option<String>>() {
                println!("{}: {}", name, val);
            }
        }
    } else {
        return get_info();
    }
    Ok(())
}

fn get_info() -> Result<()> {
    // gets the problem link
    print!("Problem link: ");
    std::io::stdout().flush()?;
    let mut link = String::from("");
    std::io::stdin().read_line(&mut link)?;

    // Maybe work on getting the information from the link itself here

    // number
    print!("Problem number: ");
    std::io::stdout().flush()?;
    let mut number = String::from("");
    std::io::stdin().read_line(&mut number)?;
    // prob_name
    print!("Problem name: ");
    std::io::stdout().flush()?;
    let mut prob_name = String::from("");
    std::io::stdin().read_line(&mut prob_name)?;
    // func_name
    print!("Function name: ");
    std::io::stdout().flush()?;
    let mut func_name = String::from("");
    std::io::stdin().read_line(&mut func_name)?;
    // args_func
    print!("Function arguments: ");
    std::io::stdout().flush()?;
    let mut args_func = String::from("");
    std::io::stdin().read_line(&mut args_func)?;
    // ret_func
    print!("Function return value: ");
    std::io::stdout().flush()?;
    let mut ret_func = String::from("");
    std::io::stdin().read_line(&mut ret_func)?;
    // extra
    print!("Extra Problem information: ");
    std::io::stdout().flush()?;
    let mut extra = String::from("");
    std::io::stdin().read_line(&mut extra)?;

    // print all the values read in

    println!("link: {}", link.trim());
    println!("number: {}", number.trim());
    println!("func_name: {}", func_name.trim());
    println!("prob_name: {}", prob_name.trim());
    println!("args_func: {}", args_func.trim());
    println!("ret_func: {}", ret_func.trim());
    println!("extra: {}", extra.trim());

    // next is to check if the [[bin]] exists in the Cargo.toml
    // if it does then exit
    // otherwise make the directory, readme, and main.rs
    // optionally make the test.rs too
    Ok(())
}
