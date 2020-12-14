use color_eyre::eyre::Result;
use owo_colors::OwoColorize;
use std::{env, fs};

use mdx::{parse, stringify};

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("It's MDX, IN RUST. THAT'S SO COOL!!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = fs::read_to_string(filename)?;
    println!("{} {} from:", "Converting".green().bold(), filename.green());
    println!("{:?}", file);
    println!("{}", "to".green());
    let parsed = parse(&file)?;
    println!("{:#?}", parsed);
    println!("{}", "stringified back:".green());
    println!("{}", stringify(parsed));
    Ok(())
}
