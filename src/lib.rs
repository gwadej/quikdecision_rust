extern crate rand;
extern crate regex;

use std::env;

mod coin;
mod pick;
mod percent;
mod dice;
mod select;
mod oracle;

pub fn parse_args(mut args: std::env::Args) -> Result<Command, String>
{
    args.next();  // discard program name
    let cmd = match args.next()
    {
        Some(c)  => c,
        None     => return Err(String::from("Missing decision type")),
    };

    match &cmd[..]
    {
        "coin"    | "flip"   => coin::command(),
        "pick"    | "choose" => pick::command(&mut args),
        "percent" | "likely" => percent::command(&mut args),
        "roll"    | "dice"   => dice::command(&mut args),
        "select"             => select::command(&mut args),
        "oracle"             => oracle::command(),
        _                    => Err(String::from("Unknown command")),
    }
}

pub enum Command
{
    CoinFlip,
    PickNumber(u32,u32),
    PercentTrue(u32),
    RollDice(Vec<dice::Roll>),
    Selection(Vec<String>),
    Oracle,
}

pub trait Decider {
    fn decide(self) -> String;
}

impl Decider for Command {
    fn decide(self) -> String
    {
        match self
        {
            Command::CoinFlip             => coin::flip(),
            Command::PickNumber(low,high) => pick::choose(low, high),
            Command::PercentTrue(likely)  => percent::choose(likely),
            Command::RollDice(expr)       => dice::roll(expr),
            Command::Selection(strvec)    => select::choose(strvec),
            Command::Oracle               => oracle::spake(),
        }
    }
}

pub fn int_arg(args: &mut env::Args) -> Result<u32, String>
{
    match args.next()
    {
        None => Err(String::from("Missing required parameter")),
        Some(arg) => match arg.parse::<u32>()
        {
            Ok(a) => Ok(a),
            Err(_) => Err(String::from("Argument not a valid integer")),
        }
    }
}
