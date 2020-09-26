use rand::thread_rng;

use crate::{Command, Decision, Decider};
use crate::ApiDoc;

const COIN_SIDES: [&str; 2] = ["Heads", "Tails"];

#[derive(Debug, Clone)]
pub struct Coin;

impl Decider for Coin {
    fn decide(&self) -> Decision {
        let mut rng = thread_rng();
        Decision::Text(super::pick_one(&mut rng, &COIN_SIDES))
    }
}

/// Create a CoinFlip Command
pub fn command() -> crate::Result<Command>
{
    Ok(Command::CoinFlip(Coin{}))
}

/// Return an ApiDoc object containing a description of the CoinFlip
/// decider.
pub fn api_doc() -> ApiDoc
{
    ApiDoc {
        name: "coin",
        params: Vec::new(),
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
    use spectral::prelude::*;

    const NUM_TRIES: u32 = 3;
    use crate::Decision;
    use crate::DecisionAssertions;
    use crate::Decider;
    use super::*;

    #[test]
    fn command_check()
    {
        assert_that!(command()).is_ok_containing(Command::CoinFlip(Coin{}));
    }

    #[test]
    fn decide_check()
    {
        assert_that!(command().unwrap().decide())
            .matches_enum_variant(Decision::Text("Heads".into()));
    }

    #[test]
    fn coin_tosses()
    {
        let expected = ["Heads", "Tails"];

        let coin = Coin{};

        for _ in 1..=NUM_TRIES
        {
            match coin.decide()
            {
                Decision::Text(flip) =>
                    assert_ne!(expected.iter().find(|&&x| x == flip), None),
                _ => assert!(false, "Wrong decision type"),
            }
        }
    }
}
