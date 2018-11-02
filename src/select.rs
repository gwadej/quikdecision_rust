use super::Command;
use rand::seq;
use std::env;
use std::fs::File;
use std::io::prelude::*;

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

pub fn hint() -> Vec<super::Hint>
{
    vec![
        super::Hint {
            clue: "select {strs}",
            blurb: "Select one of two or more strings supplied as arguments",
        },
        super::Hint {
            clue: "select @{filename}",
            blurb: "Select one of the lines in the file specified",
        },
    ]
}

fn list_from_args(first: String, args: &mut env::Args) -> Result<StrVec, String>
{
    let mut strvec = Vec::new();

    strvec.push(first);
    for a in args
    {
        strvec.push(a);
    }

    Ok(strvec)
}

fn list_from_file(filename: &str) -> Result<StrVec, String>
{
    let mut strvec = Vec::new();

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
    for a in contents.split("\n").filter(|line| line.len() > 0)
    {
        strvec.push(String::from(a));
    }

    Ok(strvec)
}

pub fn choose(strvec: StrVec) -> String
{
    let mut rng = rand::thread_rng();
    seq::sample_slice(&mut rng, &strvec, 1)[0].clone()
}
