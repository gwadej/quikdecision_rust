extern crate rand;

use rand::Rng;
use rand::seq;
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

const COIN_SIDES: [&str; 2] = [
    "Heads",
    "Tails",
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
        "coin"    | "flip"   => Ok(Command::CoinToss),
        "pick"    | "choose" => pick_command(&mut args),
        "percent" | "likely" => percent_command(&mut args),
        "roll"    | "dice"   => Ok(Command::RollDice(args.next().unwrap())),
        "oracle"             => Ok(Command::Oracle),
        _                    => Err("Unknown command"),
    }
}

fn int_arg(args: &mut env::Args) -> u32
{
    let arg = args.next().expect("Missing required parameter");
    arg.parse::<u32>().expect("Required argument not a valid integer")
}

fn pick_command(args: &mut env::Args) -> Result<Command, &'static str>
{
    let low  = int_arg(args);
    let high = int_arg(args);

    Ok(Command::PickNumber(low, high))
}

fn percent_command(args: &mut env::Args) -> Result<Command, &'static str>
{
    let likely = int_arg(args);

    Ok(Command::PercentTrue(likely))
}

fn coin_toss() -> String
{
    String::from(random_choice(&COIN_SIDES))
}

fn pick_number(low: u32, high: u32) -> String
{
    let guess = rand::thread_rng().gen_range(low, high+1);
    guess.to_string()
}

fn percent_true(likely: u32) -> String
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

fn roll_dice(_expr: String) -> String
{
    String::from("")
}

fn random_choice<'a>(choices: &'a[&str]) -> &'a str
{
    let mut rng = rand::thread_rng();
    seq::sample_slice(&mut rng, choices, 1)[0]
}

fn oracle() -> String
{
    format!("The Oracle says: \"{}\"", random_choice(&ORACLE_ANSWERS))
}
