use rand::thread_rng;

use crate::{Command, Decision, Decider};
use crate::Error;
use crate::ApiDoc;

#[derive(Debug)]
pub struct Choices(Vec<String>);

/// Create a Selection Command variant from the supplied
/// Vec of Strings.
pub fn command(strings: Vec<String>) -> crate::Result<Command>
{
    match strings.len()
    {
        0 => Err(Error::ListEmpty),
        1 => Err(Error::ListOne),
        _ => Ok(Command::Selection(Choices(strings))),
    }
}

/// Return an ApiDoc object containing a description of the Selection
/// decider.
pub fn api_doc() -> ApiDoc
{
    ApiDoc {
        name: "select",
        params: vec!["strs"],
        hint: "Select one of two or more strings supplied as arguments",
        help: vec![
            "Selects one of the supplied strings with equal probability. There must be",
            "at least two strings to choose between.",
        ],
    }
}

impl Decider for Choices {
    /// Return a Text Decision containing one of the strings from the
    /// Vec chosen at random.
    fn decide(&self) -> Decision
    {
        let mut rng = thread_rng();
        Decision::Text(super::pick_one(&mut rng, &self.0))
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
            .is_err_containing(Error::ListEmpty);
    }

    #[test]
    fn command_single_string()
    {
        assert_that!(command(vec!["fred".into()]))
            .is_err_containing(Error::ListOne);
    }

    #[test]
    fn decide_check()
    {
        let names: Vec<String> = vec!["david".into(), "mark".into(), "kirsten".into(), "connie".into()];
        assert_that!(command(names.clone()).unwrap().decide())
            .matches_enum_variant(Decision::Text("david".into()));
    }

    #[test]
    fn command_string_list()
    {
        let names: Vec<String> = vec!["david".into(), "mark".into(), "kirsten".into(), "connie".into()];
        assert_that!(command(names.clone()))
            .is_ok_containing(Command::Selection(Choices(names)));
    }

    #[test]
    fn selection_decision()
    {
        let names = vec!["david".to_string(), "mark".to_string(), "kirsten".to_string(), "connie".to_string()];
        match command(names.clone()).unwrap().decide()
        {
            Decision::Text(guess) => assert!(names.iter().any(|s| *s == guess)),
            _ => assert!(false, "Unexpected Decision"),
        }
    }
}
