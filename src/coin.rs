use ::Command;
use ::Decision;
use ::ApiDoc;

const COIN_SIDES: [&str; 2] = ["Heads", "Tails"];

/// Create a CoinFlip Command
pub fn command() -> Result<Command, String>
{
    Ok(Command::CoinFlip)
}

/// Perform the flip operation and return a Text Decision
/// a value of either "Head" or "Tails" with equal probability.
pub fn flip() -> Decision
{
    Decision::Text(super::pick_one(&COIN_SIDES))
}

/// Return an ApiDoc object containing a description of the CoinFlip
/// decider.
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

#[cfg(test)]
mod tests
{
    const NUM_TRIES: u32 = 3;
    use ::Decision;
    use ::Command;
    use super::*;

    #[test]
    fn command_check()
    {
        match command()
        {
            Ok(Command::CoinFlip) => assert!(true),
            Ok(_) => assert!(false, "Wrong Command type"),
            Err(msg) => assert!(false, "Err({})", msg),
        }
    }

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
