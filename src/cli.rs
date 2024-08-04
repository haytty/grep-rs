use anyhow::Result;
use clap::Parser;
use regex::Regex;

use crate::grep;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    recursive: bool,

    #[arg(help = "dir_path")]
    dir: String,

    #[arg(help = "regex")]
    regex: String,
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    let regex_string = format!(r"({})", args.regex);
    let regex = Regex::new(&regex_string)?;

    grep::call(args.recursive, args.dir, regex)
}