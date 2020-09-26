use crate::Command;
use crate::Decision;
use crate::Decider;
use crate::ApiDoc;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Likely(u32);

/// Create a PercentTrue Command based on the supplied percent value.
/// Returns the command or an error specifying an invald parameter.
pub fn command(likely: u32) -> Result<Command, String>
{
    match likely
    {
        0 => Err("percent arg cannot be 0".to_string()),
        num if num >= 100 => Err("percent arg cannot be 100 percent or greater".to_string()),
        num => Ok(Command::PercentTrue(Likely(num)))
    }
}

impl Decider for Likely {
    /// Return a boolean Decision with a true value likely% of the time.
    fn decide(&self) -> Decision
    {
        Decision::Bool(rand::thread_rng().gen_bool(f64::from(self.0) / 100.0))
    }
}

impl PartialEq for Likely {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
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
            "Treats the supplied integer as a percentage and returns the boolean true",
            "that percent of the time. Otherwise, return the boolean false.",
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
        assert_that!(command(0))
            .is_err_containing("percent arg cannot be 0".to_string());
    }

    #[test]
    fn command_100_percent()
    {
        assert_that!(command(100))
            .is_err_containing("percent arg cannot be 100 percent or greater".to_string());
    }

    #[test]
    fn command_gt_100_percent()
    {
        assert_that!(command(200))
            .is_err_containing("percent arg cannot be 100 percent or greater".to_string());
    }

    #[test]
    fn command_50_percent()
    {
        assert_that!(command(50))
            .is_ok_containing(Command::PercentTrue(Likely(50)));
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
        let decider = Likely(35);
        let choices: usize = (1..=1000)
            .map(|_| decider.decide())
            .filter(|x| match x { Decision::Bool(true) => true, _ => false, })
            .count();
        assert_that!(&choices).is_greater_than_or_equal_to(300);
        assert_that!(&choices).is_less_than_or_equal_to(400);
    }
}
