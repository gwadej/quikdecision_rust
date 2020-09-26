use crate::Command;
use crate::Decision;
use crate::Decider;
use crate::ApiDoc;

use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Choices(Vec<String>);

/// Create a Selection Command variant from the supplied
/// Vec of Strings.
pub fn command(strings: Vec<String>) -> Result<Command, String>
{
    match strings.len()
    {
        0 => Err("Missing required strings".to_string()),
        1 => Err("Must supply at least two strings".to_string()),
        _ => Ok(Command::Shuffle(Choices(strings))),
    }
}

/// Return an ApiDoc object containing a description of the Selection
/// decider.
pub fn api_doc() -> ApiDoc
{
    ApiDoc {
        name: "shuffle",
        params: vec!["strs"],
        hint: "Randomly re-order the supplied group of strings",
        help: vec![
            "Randomly change the order of the supplied strings. There must",
            "be at least two strings to shuffle.",
        ],
    }
}

impl Decider for Choices {
    /// Return a List containing the strings in a random order.
    fn decide(&self) -> Decision
    {
        let mut rng = thread_rng();
        let mut strvec = self.0.to_owned();
        strvec.as_mut_slice().shuffle(&mut rng);
        Decision::List(strvec)
    }
}

impl PartialEq for Choices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
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
    fn command_empty_vector()
    {
        assert_that!(command(Vec::new()))
            .is_err_containing("Missing required strings".to_string());
    }

    #[test]
    fn command_single_string()
    {
        assert_that!(command(vec!["fred".into()]))
            .is_err_containing("Must supply at least two strings".to_string());
    }

    #[test]
    fn decide_check()
    {
        let names: Vec<String> = vec!["david".into(), "mark".into(), "kirsten".into(), "connie".into()];
        assert_that!(command(names.clone()).unwrap().decide())
            .matches_enum_variant(Decision::List(names));
    }

    #[test]
    fn command_string_list()
    {
        let names: Vec<String> = vec!["david".into(), "mark".into(), "kirsten".into(), "connie".into()];
        assert_that!(command(names.clone()))
            .is_ok_containing(Command::Shuffle(Choices(names)));
    }

    #[test]
    fn selection_decision()
    {
        let names = vec!["david".to_string(), "mark".to_string(), "kirsten".to_string(), "connie".to_string()];
        match command(names.clone()).unwrap().decide()
        {
            Decision::List(guesses) => {
                assert!(guesses.len() == names.len());
                assert!(guesses.iter().all(|g| names.contains(&g)));
                assert!(names.iter().all(|g| guesses.contains(&g)));
            },
            _ => assert!(false, "Unexpected Decision"),
        }
    }
}
