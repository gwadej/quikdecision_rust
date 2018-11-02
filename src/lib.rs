extern crate rand;
extern crate regex;

use std::env;

mod coin;
mod dice;
mod oracle;
mod percent;
mod pick;
mod select;

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
        "help" => usage(progname),
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

type Hint = (
    &'static str,
    &'static str,
    Option<(&'static str, &'static str)>,
);
fn print_hint(hint: Hint)
{
    print_hint_seg(hint.0, hint.1);
    if let Some((alias, blurb)) = hint.2
    {
        print_hint_seg(alias, blurb);
    }
}

fn print_hint_seg(clue: &str, blurb: &str)
{
    if clue.len() < 8
    {
        println!("  {:8}- {}", clue, blurb);
    }
    else
    {
        println!("  {}\n  {:8}- {}", clue, "", blurb);
    }
}

fn usage(progname: String) -> !
{
    println!("{} {}\n", progname, "{command} [cmd_args ...]");
    println!("{}\n", "where {command} is one of:");
    print_hint(coin::hint());
    print_hint(pick::hint());
    print_hint(percent::hint());
    print_hint(dice::hint());
    print_hint(select::hint());
    print_hint(oracle::hint());
    print_hint(("help", "This screen", None));

    std::process::exit(1);
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
