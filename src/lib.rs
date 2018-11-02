extern crate rand;
extern crate regex;

use std::env;

mod coin;
mod dice;
mod help;
mod oracle;
mod percent;
mod pick;
mod select;

type Hint = help::Hint;

pub fn parse_args(mut args: std::env::Args) -> Result<Command, String>
{
    let progname = args.next().unwrap();
    let cmd = match args.next()
    {
        Some(c) => c,
        None => return Err(String::from("Missing decision type")),
    };

    match &cmd[..]
    {
        "coin" | "flip" => coin::command(),
        "pick" | "choose" => pick::command(&mut args),
        "percent" | "likely" => percent::command(&mut args),
        "roll" | "dice" => dice::command(&mut args),
        "select" => select::command(&mut args),
        "oracle" => oracle::command(),
        "help" => help::usage(progname, vec![
                coin::hint(),
                pick::hint(),
                percent::hint(),
                dice::hint(),
                select::hint(),
                oracle::hint(),
                help::hint(),
            ]),
        _ => Err(String::from("Unknown command")),
    }
}

pub enum Command
{
    CoinFlip,
    PickNumber(i32, i32),
    PercentTrue(u32),
    RollDice(Vec<dice::Roll>),
    Selection(Vec<String>),
    Oracle,
}

pub trait Decider
{
    fn decide(self) -> String;
}

impl Decider for Command
{
    fn decide(self) -> String
    {
        match self
        {
            Command::CoinFlip => coin::flip(),
            Command::PickNumber(low, high) => pick::choose(low, high),
            Command::PercentTrue(likely) => percent::choose(likely),
            Command::RollDice(expr) => dice::roll(expr),
            Command::Selection(strvec) => select::choose(strvec),
            Command::Oracle => oracle::spake(),
        }
    }
}

pub fn int_arg<T>(args: &mut env::Args) -> Result<T, String>
where
    T: std::str::FromStr,
{
    match args.next()
    {
        None => Err(String::from("Missing required parameter")),
        Some(arg) => match arg.parse::<T>()
        {
            Ok(a) => Ok(a),
            Err(_) => Err(String::from("Argument not a valid integer")),
        },
    }
}
