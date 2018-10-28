extern crate rand;
extern crate regex;

use std::env;
use rand::seq;

pub enum Command
{
    CoinFlip,
    PickNumber(u32,u32),
    PercentTrue(u32),
    RollDice(Vec<dice::Roll>),
    Oracle,
}

pub trait Decider {
    fn decide(self) -> String;
}

pub fn parse_args(mut args: std::env::Args) -> Result<Command, String>
{
    args.next();  // discard program name
    let cmd = args.next().expect("Missing decision type");
    match &cmd[..]
    {
        "coin"    | "flip"   => coin::command(),
        "pick"    | "choose" => pick::command(&mut args),
        "percent" | "likely" => percent::command(&mut args),
        "roll"    | "dice"   => dice::command(&mut args),
        "oracle"             => oracle::command(),
        _                    => Err(String::from("Unknown command")),
    }
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

fn random_choice<'a>(choices: &'a[&str]) -> &'a str
{
    let mut rng = rand::thread_rng();
    seq::sample_slice(&mut rng, choices, 1)[0]
}

mod pick;
mod percent;
mod dice;
mod coin;
mod oracle;

#[cfg(test)]
mod tests
{
    const NUM_TRIES: u32 = 3;

    #[test]
    fn coin_tosses()
    {
        let expected = ["Heads", "Tails"];

        for _ in 1..=NUM_TRIES
        {
            let flip = super::coin_toss();
            assert_ne!(expected.iter().find(|&&x| x == flip), None);
        }
    }

    #[test]
    fn choose_a_small_number()
    {
        let expected = ["1", "2"];

        for _ in 1..=NUM_TRIES
        {
            let choice = super::pick_number(1, 2);
            assert_ne!(expected.iter().find(|&&x| x == choice), None);
        }
    }

    #[test]
    fn choose_a_larger_number()
    {
        let low: u32 = 2;
        let high: u32 = 10;
        let expected = [
            "2", "3", "4", "5", "6",
            "7", "8", "9", "10",
        ];

        for _ in 1..=NUM_TRIES
        {
            let choice = super::pick_number(low, high);
            assert_ne!(expected.iter().find(|&&x| x == choice), None);
        }
    }

    #[test]
    fn percent_test()
    {
        let choices: usize = (1..=1000)
            .map(|_| super::percent_true(35))
            .filter(|x| x == "True")
            .count();
        assert!(300 <= choices && choices <= 400);
    }
}
