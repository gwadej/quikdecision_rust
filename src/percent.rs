use crate::Command;
use crate::Decision;
use crate::ApiDoc;

use rand::Rng;

/// Create a PercentTrue Command based on the supplied percent value.
/// Returns the command or an error specifying an invald parameter.
pub fn command(likely: u32) -> Result<Command, String>
{
    match likely
    {
        0 => Err(String::from("percent arg cannot be 0")),
        num if num >= 100 => Err(String::from("percent arg cannot be 100 percent or greater")),
        num => Ok(Command::PercentTrue(num))
    }
}

/// Return a boolean Decision with a true value likely% of the time.
pub fn choose(likely: u32) -> Decision
{
    Decision::Bool(rand::thread_rng().gen_bool(likely as f64 / 100.0))
}

/// Return an ApiDoc object containing a description of the PercentTrue
/// decider.
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

#[cfg(test)]
mod tests
{
    use spectral::prelude::*;

    use crate::Decision;
    use crate::DecisionAssertions;
    use crate::Decider;
    use crate::Command;
    use super::*;

    #[test]
    fn command_0_percent()
    {
        assert_that!(command(0)).is_err()
            .is_equal_to("percent arg cannot be 0".to_string());
    }

    #[test]
    fn command_100_percent()
    {
        assert_that!(command(100)).is_err()
            .is_equal_to("percent arg cannot be 100 percent or greater".to_string());
    }

    #[test]
    fn command_gt_100_percent()
    {
        assert_that!(command(200)).is_err()
            .is_equal_to("percent arg cannot be 100 percent or greater".to_string());
    }

    #[test]
    fn command_50_percent()
    {
        assert_that!(command(50)).is_ok()
            .is_equal_to(Command::PercentTrue(50));
    }

    #[test]
    fn decide_check()
    {
        assert_that!(command(45).unwrap().decide())
            .matches_enum_variant(Decision::Bool(true));
    }

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
