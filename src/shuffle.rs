use crate::Command;
use crate::Decision;
use crate::ApiDoc;

use rand::Rng;
use rand::seq::SliceRandom;

type StrVec = Vec<String>;

/// Create a Selection Command variant from the supplied
/// Vec of Strings.
pub fn command(strings: StrVec) -> Result<Command, String>
{
    if strings.is_empty()
    {
        return Err(String::from("Missing required strings"));
    }

    if strings.len() > 1
    {
        Ok(Command::Shuffle(strings))
    }
    else
    {
        Err(String::from("Must supply at least two strings"))
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

/// Return a List containing the strings in a random order.
pub fn order(strvec: StrVec) -> Decision
{
    let mut rng = thread_rng();
    Decision::List(strvec.as_slice().shuffle(&mut rng))
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
        assert_that!(command(Vec::new())).is_err()
            .is_equal_to("Missing required strings".to_string());
    }

    #[test]
    fn command_single_string()
    {
        assert_that!(command(vec!["fred".into()])).is_err()
            .is_equal_to("Must supply at least two strings".to_string());
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
            .is_ok()
            .is_equal_to(Command::Selection(names));
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
