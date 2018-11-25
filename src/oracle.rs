use ::Command;
use ::Decision;
use ::ApiDoc;

const ORACLE_ANSWERS: [&str; 24] = [
    // Positive answeers
    "It is certain",
    "It is decidedly so",
    "So it is written",
    "Most likely",
    "Outlook good",
    "Signs point to yes",
    "Without a doubt",
    "Yes",
    "You may rely on it",
    // Negative answers
    "Don't count on it",
    "My reply is no",
    "My sources say no",
    "No",
    "Not a chance",
    "Outlook not so good",
    "Very doubtful",
    "You must be joking",
    "The spirits say no",
    // Unknown answers
    "Ask again later",
    "Cannot predict now",
    "Concentrate and ask again",
    "Reply hazy, try again",
    "The future is uncertain",
    "I have no answer at this time",
];

const ORACLE_LABELS: [&str; 10] = [
    "Thus spoke the Oracle",
    "Thus sayeth the Oracle",
    "The Oracle says",
    "The Oracle intoned",
    "On this day, the Oracle says",
    "It is commonly said",
    "The voices told me to say",
    "The elder said",
    "Would you believe",
    "The fortune cookie says",
];

/// Return an ApiDoc object containing a description of the Oracle
/// decider.
pub fn api_doc() -> ApiDoc
{
    ApiDoc {
        name: "oracle",
        params: vec![],
        hint: "Return a random answer from the oracle",
        help: vec![
            "This command returns a string randomly selected from the Oracle's set of",
            "9 positive answers, 9 negative answers, or 6 indeterminate answers.",
        ],
    }
}

/// Construct an Oracle Command variant
pub fn command() -> Result<Command, String>
{
    Ok(Command::Oracle)
}

/// Perform the actual decision for the Oracle and return the Decision.
pub fn choose() -> Decision
{
    Decision::LabelledText{
        value: ::pick_one(&ORACLE_ANSWERS).to_string(),
        label: ::pick_one(&ORACLE_LABELS).to_string(),
    }
}

#[cfg(test)]
mod tests
{
    use spectral::prelude::*;

    use ::Decision;
    use ::DecisionAssertions;
    use ::Decider;
    use ::Command;
    use super::*;

    #[test]
    fn command_check()
    {
        assert_that!(command()).is_ok()
            .is_equal_to(Command::Oracle);
    }

    #[test]
    fn decision_check()
    {
        assert_that!(command().unwrap().decide())
            .matches_enum_variant(Decision::LabelledText{value: "foo".into(), label: "bar".into()});
    }
}
