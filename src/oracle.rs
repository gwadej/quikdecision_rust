use ::Command;
use ::Decision;

use help;

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

pub fn hint() -> Vec<help::Hint>
{
    vec![help::Hint {
        cmd: "oracle",
        clue: "oracle",
        blurb: "Return a random answer from the oracle",
        help: vec![
            "This command returns a string randomly selected from the Oracle's set of",
            "9 positive answers, 9 negative answers, or 6 indeterminate answers.",
        ],
    }]
}

pub fn command() -> Result<Command, String>
{
    Ok(Command::Oracle)
}

pub fn choose() -> Decision
{
    Decision::Text(super::pick_one(&ORACLE_ANSWERS).to_string())
}

pub fn spake() -> Decision
{
    let ans = match choose()
    {
        Decision::Text(ans) => ans,
        _ => panic!(),
    };
    Decision::Text(format!("Thus spoke the Oracle: \"{}\"", ans))
}
