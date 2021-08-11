use crate::{Command, Decision, Decider};
use crate::Error;
use crate::ApiDoc;

use std::cmp::Ordering;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub struct Picker {
    low: i32,
    high: i32
}

/// Create a PickNumber command based on the two supplied values
/// Return either the command or an error if the parameters are not appropriate.
pub fn command(low: i32, high: i32) -> crate::Result<Command>
{
    match low.cmp(&high)
    {
        Ordering::Equal   => Err(Error::EmptyRange),
        Ordering::Greater => Ok(Command::PickNumber(Picker{low: high, high: low})),
        Ordering::Less    => Ok(Command::PickNumber(Picker{low, high})),
    }
}

impl Decider for Picker {
    /// Return a numeric Decision with a value between low and high (inclusive).
    fn decide(&self) -> Decision
    {
        Decision::Num(rand::thread_rng().gen_range(self.low, self.high + 1))
    }
}

/// Return an ApiDoc object containing a description of the PickNumber
/// decider.
pub fn api_doc() -> ApiDoc
{
    ApiDoc {
        name: "pick",
        params: vec!["low", "high"],
        hint: "pick a number between {low} and {high}",
        help: vec![
            "Selects a number between two supplied values (inclusive) with equal probability.",
            "The two numbers cannot be the same.",
        ],
    }
}

#[cfg(test)]
mod tests
{
    use spectral::prelude::*;

    const NUM_TRIES: u32 = 3;
    use crate::Decision;
    use crate::DecisionAssertions;
    use crate::Decider;
    use crate::Command;
    use super::*;

    #[test]
    fn command_with_equal_params()
    {
        assert_that!(command(1, 1))
            .is_err_containing(Error::EmptyRange);
    }

    #[test]
    fn command_args_in_wrong_order()
    {
        assert_that!(command(30, 20))
            .is_ok_containing(Command::PickNumber(Picker{low: 20, high: 30}));
    }

    #[test]
    fn command_args_in_correct_order()
    {
        assert_that!(command(10, 20))
            .is_ok_containing(Command::PickNumber(Picker{low: 10, high: 20}));
    }

    #[test]
    fn decide_check()
    {
        assert_that!(command(1, 10).unwrap().decide())
            .is_num_decision();
    }

    #[test]
    fn choose_a_small_number()
    {
        let expected = [1, 2];

        let decider = Picker{low: 1, high: 2};
        for _ in 1..=NUM_TRIES
        {
            match decider.decide()
            {
                Decision::Num(choice) => assert_ne!(expected.iter().find(|&&x| x == choice), None),
                _ => assert!(false, "Wrong decision type"),
            }
        }
    }

    #[test]
    fn choose_a_larger_number()
    {
        let low: i32 = 2;
        let high: i32 = 10;
        let expected = [2, 3, 4, 5, 6, 7, 8, 9, 10];

        let decider = Picker{low, high};
        for _ in 1..=NUM_TRIES
        {
            match decider.decide()
            {
                Decision::Num(choice) => assert_ne!(expected.iter().find(|&&x| x == choice), None),
                _ => assert!(false, "Wrong decision type"),
            }
        }
    }
}
