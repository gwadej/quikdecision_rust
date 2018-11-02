use super::Command;
use rand::seq;

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

pub fn hint() -> super::Hint
{
    ("oracle", "Return a random answer from the oracle", None)
}

pub fn command() -> Result<Command, String>
{
    Ok(Command::Oracle)
}

pub fn spake() -> String
{
    let mut rng = rand::thread_rng();
    let ans = seq::sample_slice(&mut rng, &ORACLE_ANSWERS, 1)[0];
    format!("Thus spoke the Oracle: \"{}\"", ans)
}
