use ::Command;
use ::Decision;
use ::Hint;
use ::HintList;
use ::ApiDoc;

use rand::Rng;

/// Create a PercentTrue Command based on the supplied percent value.
/// Returns the command or an error specifying an invald parameter.
pub fn command(likely: u32) -> Result<Command, String>
{
    match likely
    {
        0 => Err(String::from("percent arg cannot be 0")),
        num if num > 100 => Err(String::from("percent arg cannot be greater than 100 percent")),
        num => Ok(Command::PercentTrue(num))
    }
}

/// Return a boolean Decision with a true value likely% of the time.
pub fn choose(likely: u32) -> Decision
{
    Decision::Bool(rand::thread_rng().gen_bool(likely as f64 / 100.0))
}

pub fn api_doc() -> ApiDoc
{
    ApiDoc {
        name: "percent",
        params: vec!["num"],
        hint: "True {num} percent of the time, otherwise False",
        help: vec![
            "Treats the supplied integer as a percentage and returns the string 'True'",
            "that percent of the time. Otherwise, return the string 'False'.",
        ],
    }
}

pub fn hint() -> HintList
{
    vec![
        Hint {
            cmd: "percent",
            clue: "percent {num}",
            blurb: "True {num} percent of the time, otherwise False",
            help: vec![
                "Treats the supplied integer as a percentage and returns the string 'True'",
                "that percent of the time. Otherwise, return the string 'False'.",
            ],
        },
        Hint {
            cmd: "likely",
            clue: "likely {num}",
            blurb: "alias for percent",
            help: vec![
                "Treats the supplied integer as a percentage and returns the string 'True'",
                "that percent of the time. Otherwise, return the string 'False'.",
            ],
        },
    ]
}

#[cfg(test)]
mod tests
{
    use ::Decision;
    #[test]
    fn percent_test()
    {
        let choices: usize = (1..=1000)
            .map(|_| super::choose(35))
            .filter(|x| match x { Decision::Bool(true) => true, _ => false, })
            .count();
        assert!(300 <= choices && choices <= 400);
    }
}
