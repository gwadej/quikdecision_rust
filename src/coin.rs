use ::Command;
use ::Decision;
use ::ApiDoc;

const COIN_SIDES: [&str; 2] = ["Heads", "Tails"];

pub fn command() -> Result<Command, String>
{
    Ok(Command::CoinFlip)
}

pub fn flip() -> Decision
{
    Decision::Text(super::pick_one(&COIN_SIDES))
}

pub fn api_doc() -> ApiDoc
{
    ApiDoc {
        name: "coin",
        params: vec![],
        hint: "50% chance of a Heads or Tails",
        help: vec![
            "Returns one of the two strings 'Heads' or 'Tails'. There is an equal probability",
            "of returning either one.",
        ],
    }
}

//pub fn hint() -> HintList
//{
//    vec![
//        Hint {
//            cmd: "flip",
//            clue: "flip",
//            blurb: "50% chance of a Heads or Tails",
//            help: vec![
//                "Returns one of the two strings 'Heads' or 'Tails'. There is an equal probability",
//                "of returning either one.",
//            ],
//        },
//        Hint {
//            cmd: "coin",
//            clue: "coin",
//            blurb: "alias for flip",
//            help: vec![
//                "Returns one of the two strings 'Heads' or 'Tails'. There is an equal probability",
//                "of returning either one.",
//            ],
//        },
//    ]
//}

#[cfg(test)]
mod tests
{
    const NUM_TRIES: u32 = 3;
    use ::Decision;

    #[test]
    fn coin_tosses()
    {
        let expected = ["Heads", "Tails"];

        for _ in 1..=NUM_TRIES
        {
            match super::flip()
            {
                Decision::Text(flip) =>
                    assert_ne!(expected.iter().find(|&&x| x == flip), None),
                _ => panic!(),
            }
        }
    }
}
