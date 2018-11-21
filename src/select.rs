use ::Command;
use ::Decision;
use ::ApiDoc;

type StrVec = Vec<String>;

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

//pub fn hint() -> HintList
//{
//    vec![
//        Hint {
//            cmd: "select",
//            clue: "select {strs}",
//            blurb: "Select one of two or more strings supplied as arguments",
//            help: vec![
//                "Selects one of the supplied strings with equal probability. There must be",
//                "at least two strings to choose between.",
//            ],
//        },
//        Hint {
//            cmd: "select",
//            clue: "select @{filename}",
//            blurb: "Select one of the lines in the file specified",
//            help: vec![
//                "Loads a series of strings from the specified file. (Each line is one string.)",
//                "Selects one of the supplied strings with equal probability. There must be",
//                "at least two strings to choose between.",
//            ],
//        },
//    ]
//}

pub fn choose(strvec: StrVec) -> Decision
{
    Decision::Text(super::pick_one(&strvec[..]))
}
