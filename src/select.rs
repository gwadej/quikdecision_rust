use crate::Command;
use crate::Decision;
use crate::ApiDoc;

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
        Ok(Command::Selection(strings))
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
        name: "select",
        params: vec!["strs"],
        hint: "Select one of two or more strings supplied as arguments",
        help: vec![
            "Selects one of the supplied strings with equal probability. There must be",
            "at least two strings to choose between.",
        ],
    }
}

/// Return a Text Decision containing one of the strings from the
/// Vec chosen at random.
pub fn choose(strvec: StrVec) -> Decision
{
    Decision::Text(super::pick_one(strvec.as_slice()))
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
            .matches_enum_variant(Decision::Text("david".into()));
    }

    #[test]
    fn command_string_list()
    {
        let names: Vec<String> = vec!["david".into(), "mark".into(), "kirsten".into(), "connie".into()];
        assert_that!(command(names.clone()))
            .is_ok_containing(Command::Selection(names));
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
