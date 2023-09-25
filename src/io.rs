use anyhow::Result;
use std::io::Write;

#[allow(dead_code)]
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
