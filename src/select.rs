use ::Command;
use ::Decision;
use ::ApiDoc;

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
    Decision::Text(super::pick_one(&strvec[..]))
}

#[cfg(test)]
mod tests
{
    use ::Decision;
    use ::Decider;
    use ::Command;
    use super::*;

    #[test]
    fn command_empty_vector()
    {
        match command(Vec::new())
        {
            Ok(_) => assert!(false, "Unexpected Command"),
            Err(msg) => assert_eq!(msg, "Missing required strings".to_string()),
        }
    }

    #[test]
    fn command_single_string()
    {
        match command(vec!["fred".to_string()])
        {
            Ok(_) => assert!(false, "Unexpected Command"),
            Err(msg) => assert_eq!(msg, "Must supply at least two strings".to_string()),
        }
    }

    #[test]
    fn command_string_list()
    {
        match command(vec!["david".to_string(), "mark".to_string(), "kirsten".to_string(), "connie".to_string()])
        {
            Ok(Command::Selection(_)) => assert!(true),
            Ok(_) => assert!(false, "Unexpected Command"),
            Err(_) => assert!(false, "Unexpected error"),
        }
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
