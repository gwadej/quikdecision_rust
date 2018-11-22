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
