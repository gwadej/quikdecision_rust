use ::Command;
use ::Decision;
use ::Hint;
use ::HintList;
use ::int_arg;

use rand::Rng;

pub fn command(likely: Option<String>) -> Result<Command, String>
{
    match int_arg::<u32>(likely)
    {
        Ok(likely) if likely > 100 =>
            return Err(String::from("percent arg cannot be greater than 100 percent")),
        Ok(likely) => Ok(Command::PercentTrue(likely)),
        Err(e) => return Err(format!("percentage: {}", e)),
    }
}

pub fn choose(likely: u32) -> Decision
{
    Decision::Bool(rand::thread_rng().gen_bool(likely as f64 / 100.0))
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
    #[test]
    fn percent_test()
    {
        let choices: usize = (1..=1000)
            .map(|_| super::choose(35))
            .filter(|x| x == "True")
            .count();
        assert!(300 <= choices && choices <= 400);
    }
}
