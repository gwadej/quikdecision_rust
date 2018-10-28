extern crate rand;

use rand::Rng;

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
    println!("coin toss: {}",    coin_toss());
    println!("pick number: {}",  pick_number(1, 32));
    println!("percent true: {}", percent_true(30));
    println!("roll dice: {}",    roll_dice(""));
    println!("The Oracle: {}",   oracle());
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
    let guess = rand::thread_rng().gen_range(low, high);
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

fn roll_dice(_expr: &str) -> String
{
    String::from("")
}

fn oracle() -> String
{
    let index = rand::thread_rng().gen_range(0,ORACLE_ANSWERS.len()-1);
    String::from(ORACLE_ANSWERS[index])
}
