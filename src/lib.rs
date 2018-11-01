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
    let progname = args.next().unwrap();
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
        "help"               => usage(progname),
        _                    => Err(String::from("Unknown command")),
    }
}

pub enum Command
{
    CoinFlip,
    PickNumber(i32,i32),
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

fn usage(progname: String) -> !
{
    println!("{} {}\n", progname, "{command} [cmd_args ...]");
    println!("{}\n", "where {command} is one of:");
    println!("{}", "  flip    - 50% chance of a Heads or Tails");
    println!("{}", "  coin    - alias for flip");
    println!("{}", "  pick {low} {high}");
    println!("{}", "          - pick a number between {low} and {high}");
    println!("{}", "  choose  - alias for pick");
    println!("{}", "  percent {num}");
    println!("{}", "          - True {num} percent of the time, otherwise False");
    println!("{}", "  likely  - alias for percent");
    println!("{}", "  roll {dice expr}");
    println!("{}", "          - Roll the described combination of dice, returning a number");
    println!("{}", "            {dice expr} is a combination of terms of the form {n}d{s}");
    println!("{}", "            where {n} is a positive integer, {s} is a number of sides");
    println!("{}", "            for the dice (4, 6, 8, 10, 12, 20, or 100). A term can also");
    println!("{}", "            be just an integer. (e.g. 2d6+1d4+2)");
    println!("{}", "  dice    - alias for roll");
    println!("{}", "  select {strs}");
    println!("{}", "          - Select one of two or more string supplied as arguments");
    println!("{}", "  select @{filename}");
    println!("{}", "          - Select one of the lines in the file specified");
    println!("{}", "  oracle  - Return a random answer from the oracle");
    println!("{}", "  help    - This screen");

    std::process::exit(1);
}

pub fn int_arg<T>(args: &mut env::Args) -> Result<T, String>
    where T: std::str::FromStr
{
    match args.next()
    {
        None => Err(String::from("Missing required parameter")),
        Some(arg) => match arg.parse::<T>()
        {
            Ok(a) => Ok(a),
            Err(_) => Err(String::from("Argument not a valid integer")),
        }
    }
}
