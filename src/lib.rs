extern crate rand;

use std::env;
use rand::Rng;
use rand::seq;

pub enum Command
{
    CoinToss,
    PickNumber(u32,u32),
    PercentTrue(u32),
    RollDice(String),
    Oracle,
}

pub trait Decider {
    fn decide(self) -> String;
}

const ORACLE_ANSWERS: [&str; 24] = [
    // Positive answeers
    "It is certain",
    "It is decidedly so",
    "So it is written",
    "Most likely",
    "Outlook good",
    "Signs point to yes",
    "Without a doubt",
    "Yes",
    "You may rely on it",
    // Negative answers
    "Don't count on it",
    "My reply is no",
    "My sources say no",
    "No",
    "Not a chance",
    "Outlook not so good",
    "Very doubtful",
    "You must be joking",
    "The spirits say no",
    // Unknown answers
    "Ask again later",
    "Cannot predict now",
    "Concentrate and ask again",
    "Reply hazy, try again",
    "The future is uncertain",
	"I have no answer at this time",
];

const COIN_SIDES: [&str; 2] = [ "Heads", "Tails" ];

pub fn parse_args(mut args: std::env::Args) -> Result<Command, String>
{
    args.next();  // discard program name
    let cmd = args.next().expect("Missing decision type");
    match &cmd[..]
    {
        "coin"    | "flip"   => Ok(Command::CoinToss),
        "pick"    | "choose" => pick_command(&mut args),
        "percent" | "likely" => percent_command(&mut args),
        "roll"    | "dice"   => Ok(Command::RollDice(args.next().unwrap())),
        "oracle"             => Ok(Command::Oracle),
        _                    => Err(String::from("Unknown command")),
    }
}

impl Decider for Command {
    fn decide(self) -> String
    {
        match self
        {
            Command::CoinToss             => coin_toss(),
            Command::PickNumber(low,high) => pick_number(low, high),
            Command::PercentTrue(likely)  => percent_true(likely),
            Command::RollDice(expr)       => roll_dice(expr),
            Command::Oracle               => oracle(),
        }
    }
}

fn int_arg(args: &mut env::Args) -> Result<u32, String>
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

fn pick_command(args: &mut env::Args) -> Result<Command, String>
{
    let low = match int_arg(args)
    {
        Ok(val) => val,
        Err(e)  => return Err(format!("low arg: {}", e)),
    };
    let high = match int_arg(args)
    {
        Ok(val) => val,
        Err(e)  => return Err(format!("high arg: {}", e)),
    };

    Ok(Command::PickNumber(low, high))
}

fn percent_command(args: &mut env::Args) -> Result<Command, String>
{
    match int_arg(args)
    {
        Ok(likely) => Ok(Command::PercentTrue(likely)),
        Err(e)     => Err(format!("likely arg: {}", e)),
    }
}

pub fn coin_toss() -> String
{
    String::from(random_choice(&COIN_SIDES))
}

pub fn pick_number(low: u32, high: u32) -> String
{
    let guess = rand::thread_rng().gen_range(low, high+1);
    guess.to_string()
}

pub fn percent_true(likely: u32) -> String
{
    let ans = if rand::thread_rng().gen_bool(likely as f64 / 100.0)
    {
        "True"
    }
    else
    {
        "False"
    };
    String::from(ans)
}

pub fn roll_dice(_expr: String) -> String
{
    String::from("")
}

fn random_choice<'a>(choices: &'a[&str]) -> &'a str
{
    let mut rng = rand::thread_rng();
    seq::sample_slice(&mut rng, choices, 1)[0]
}

pub fn oracle() -> String
{
    format!("The Oracle says: \"{}\"", random_choice(&ORACLE_ANSWERS))
}

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
}
