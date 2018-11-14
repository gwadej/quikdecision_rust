use ::Command;
use ::Decision;

use help;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::once;

type StrVec = Vec<String>;

pub fn command(args: &mut env::Args) -> Result<Command, String>
{
    let first = match args.next()
    {
        Some(s) => s,
        None => return Err(String::from("Missing required strings")),
    };

    let strvec = if first.starts_with("@")
    {
        list_from_file(&first[1..])?
    }
    else
    {
        list_from_args(first, args)?
    };

    if strvec.len() > 1
    {
        Ok(Command::Selection(strvec))
    }
    else
    {
        Err(String::from("Must supply at least two strings"))
    }
}

pub fn hint() -> Vec<help::Hint>
{
    vec![
        help::Hint {
            cmd: "select",
            clue: "select {strs}",
            blurb: "Select one of two or more strings supplied as arguments",
            help: vec![
                "Selects one of the supplied strings with equal probability. There must be",
                "at least two strings to choose between.",
            ],
        },
        help::Hint {
            cmd: "select",
            clue: "select @{filename}",
            blurb: "Select one of the lines in the file specified",
            help: vec![
                "Loads a series of strings from the specified file. (Each line is one string.)",
                "Selects one of the supplied strings with equal probability. There must be",
                "at least two strings to choose between.",
            ],
        },
    ]
}

fn list_from_args(first: String, args: &mut env::Args) -> Result<StrVec, String>
{
    Ok(once(first).chain(args).collect::<Vec<String>>())
}

fn list_from_file(filename: &str) -> Result<StrVec, String>
{
    let mut file = match File::open(filename)
    {
        Ok(f) => f,
        Err(_) => return Err(String::from("Cannot open supplied file")),
    };
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents)
    {
        return Err(String::from("Cannot read supplied file"));
    }
    Ok(contents.split("\n")
               .filter(|line| !line.is_empty())
               .map(|s| s.to_string())
               .collect::<Vec<String>>())
}

pub fn choose(strvec: StrVec) -> Decision
{
    Decision::Text(super::pick_one(&strvec[..]))
}
