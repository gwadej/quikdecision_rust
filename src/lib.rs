extern crate rand;
extern crate regex;

use std::env;
use rand::Rng;
use rand::seq;
use regex::Regex;

pub enum Command
{
    CoinToss,
    PickNumber(u32,u32),
    PercentTrue(u32),
    RollDice(Vec<Roll>),
    Oracle,
}

pub enum Roll
{
    Dice(u32, u32),
    Incr(u32),
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
        "roll"    | "dice"   => roll_command(&mut args),
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

fn roll_command(args: &mut env::Args) -> Result<Command, String>
{
    let re = Regex::new(r"^\s*([0-9]+)[dD]([0-9]+)(?:\+([0-9]+))?$").unwrap();
    let expr = match args.next()
    {
        Some(e) => e,
        None    => return Err(String::from("Missing dice expression")),
    };
    let cap = re.captures(&expr).unwrap();
    let num_dice  = match cap.get(1)
    {
        None    => return Err(String::from("No dice specified")),
        Some(n) => n.as_str().parse::<u32>().expect("Non-number somehow passed parsing"),
    };

    let num_sides = match cap.get(2)
    {
        None    => return Err(String::from("No sides specified")),
        Some(n) => n.as_str().parse::<u32>().expect("Non-number somehow passed parsing"),
    };
    let mut descr = vec![Roll::Dice(num_dice, num_sides)];

    if let Some(num_incr)  = cap.get(3)
    {
        descr.push( Roll::Incr(num_incr.as_str().parse::<u32>().expect("Non-number somehow passed parsing")));
    }

    Ok(Command::RollDice(descr))
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

fn sub_roll(num: u32, sides: u32) -> u32
{
    if num == 0 || sides == 0
    {
        return 0
    }

    let mut rng = rand::thread_rng();
    (1..=num)
        .map(|_| rng.gen_range(1, sides+1))
        .sum::<u32>()
}

pub fn roll_dice(descr: Vec<Roll>) -> String
{

    let value = descr.iter()
        .map(|ref x| match x
             {
                 Roll::Dice(num,sides) => sub_roll(*num, *sides),
                 Roll::Incr(num)       => *num,
             }
        )
        .sum::<u32>();
    value.to_string()
}

fn random_choice<'a>(choices: &'a[&str]) -> &'a str
{
    let mut rng = rand::thread_rng();
    seq::sample_slice(&mut rng, choices, 1)[0]
}

pub fn oracle() -> String
{
    format!("Thus spoke the Oracle: \"{}\"", random_choice(&ORACLE_ANSWERS))
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
