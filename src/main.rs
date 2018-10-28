extern crate rand;

use rand::Rng;
use std::env;

enum Command
{
    CoinToss,
    PickNumber(u32,u32),
    PercentTrue(u32),
    RollDice(String),
    Oracle,
}

const ORACLE_ANSWERS: [&str; 24] = [
    "It is certain",
    "It is decidedly so",
    "So it is written",
    "Most likely",
    "Outlook good",
    "Signs point to yes",
    "Without a doubt",
    "Yes",
    "You may rely on it",
    "Don't count on it",
    "My reply is no",
    "My sources say no",
    "No",
    "Not a chance",
    "Outlook not so good",
    "Very doubtful",
    "You must be joking",
    "The spirits say no",
    "Ask again later",
    "Cannot predict now",
    "Concentrate and ask again",
    "Reply hazy, try again",
    "The future is uncertain",
	"I have no answer at this time",
];

fn main()
{
    let cmd = match parse_args(env::args())
    {
        Ok(c) => c,
        Err(m) => panic!(m)
    };

    let output = match cmd
    {
        Command::CoinToss             => coin_toss(),
        Command::PickNumber(low,high) => pick_number(low, high),
        Command::PercentTrue(likely)  => percent_true(likely),
        Command::RollDice(expr)       => roll_dice(expr),
        Command::Oracle               => oracle(),
    };

    println!("{}", output);
}

fn parse_args(mut args: std::env::Args) -> Result<Command, &'static str>
{
    args.next();  // discard program name
    let cmd = args.next().expect("Missing decision type");
    match &cmd[..]
    {
        "coin"    => Ok(Command::CoinToss),
        "pick"    => Ok(Command::PickNumber(int_arg(args.next()), int_arg(args.next()))),
        "percent" => Ok(Command::PercentTrue(int_arg(args.next()))),
        "oracle"  => Ok(Command::Oracle),
        "roll"    => Ok(Command::RollDice(args.next().unwrap())),
        _         => Err("Unknown command"),
    }
}

fn int_arg(arg: Option<String>) -> u32
{
    arg.unwrap().parse::<u32>().unwrap()
}

fn coin_toss() -> String
{
    if rand::thread_rng().gen::<f64>() < 0.5
    {
        String::from("Heads")
    }
    else
    {
        String::from("Tails")
    }
}

fn pick_number(low: u32, high: u32) -> String
{
    let guess = rand::thread_rng().gen_range(low, high+1);
    guess.to_string()
}

fn percent_true(likely: u32) -> String
{
    if rand::thread_rng().gen::<f64>() < (likely as f64 / 100.0)
    {
        String::from("True")
    }
    else
    {
        String::from("False")
    }
}

fn roll_dice(_expr: String) -> String
{
    String::from("")
}

fn oracle() -> String
{
    let index = rand::thread_rng().gen_range(0,ORACLE_ANSWERS.len()-1);
    String::from(ORACLE_ANSWERS[index])
}
