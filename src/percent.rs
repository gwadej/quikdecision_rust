use crate::{Command, Decision, Decider};
use crate::Error;
use crate::ApiDoc;

use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Likely(u32);

/// Create a PercentTrue Command based on the supplied percent value.
/// Returns the command or an error specifying an invald parameter.
pub fn command(likely: u32) -> crate::Result<Command>
{
    match likely
    {
        0 => Err(Error::PercentZero),
        num if num >= 100 => Err(Error::PercentOverflow(num)),
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
            .is_err_containing(Error::PercentZero);
    }

    #[test]
    fn command_100_percent()
    {
        assert_that!(command(100))
            .is_err_containing(Error::PercentOverflow(100));
    }

    #[test]
    fn command_gt_100_percent()
    {
        assert_that!(command(200))
            .is_err_containing(Error::PercentOverflow(200));
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
