extern crate rand;
extern crate regex;

use std::env;
use rand::seq;

mod coin;
mod dice;
mod help;
mod oracle;
mod percent;
mod pick;
mod select;

pub enum Command
{
    CoinFlip,
    PickNumber(i32, i32),
    PercentTrue(u32),
    RollDice(Vec<dice::Roll>),
    Selection(Vec<String>),
    Oracle,
}

pub enum Decision
{
    Text(String),
    LabeledText{ value: String, label: String },
    Num(i32),
    AnnotatedNum{ value: u32, extra: String },
    Bool(bool),
}

pub trait Decider
{
    fn decide(self) -> Decision;
}

impl Decider for Command
{
    fn decide(self) -> Decision
    {
        match self
        {
            Command::CoinFlip => coin::flip(),
            Command::PickNumber(low, high) => pick::choose(low, high),
            Command::PercentTrue(likely) => percent::choose(likely),
            Command::RollDice(expr) => dice::roll(expr),
            Command::Selection(strvec) => select::choose(strvec),
            Command::Oracle => oracle::choose(),
        }
    }
}

pub fn parse_args(mut args: std::env::Args) -> Result<Command, String>
{
    let progname = args.next().unwrap();
    let cmd = match args.next()
    {
        Some(c) => c,
        None => return Err(String::from("Missing decision type")),
    };
    let all_hints = vec![
        coin::hint(),
        pick::hint(),
        percent::hint(),
        dice::hint(),
        select::hint(),
        oracle::hint(),
        help::hint(),
    ];

    match &cmd[..]
    {
        "coin" | "flip" => coin::command(),
        "pick" => pick::command(&mut args),
        "percent" | "likely" => percent::command(&mut args),
        "roll"  => dice::command(&mut args),
        "select" => select::command(&mut args),
        "oracle" => oracle::command(),
        "help" => help::usage(progname, args.next(), all_hints),
        "man" => help::help(progname, args.next(), all_hints),
        _ => Err(String::from("Unknown command")),
    }
}

pub fn pick_one<T>(choices: &[T]) -> String
    where T : std::string::ToString + std::clone::Clone
{
    let mut rng = rand::thread_rng();
    seq::sample_slice(&mut rng, choices, 1)[0].to_string()
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
