use help;
use rand::seq;

use super::Command;

const COIN_SIDES: [&str; 2] = ["Heads", "Tails"];

pub fn command() -> Result<Command, String>
{
    Ok(Command::CoinFlip)
}

pub fn flip() -> String
{
    let mut rng = rand::thread_rng();
    let ans = seq::sample_slice(&mut rng, &COIN_SIDES, 1)[0];

    String::from(ans)
}

pub fn hint() -> Vec<help::Hint>
{
    vec![
        help::Hint {
            cmd: "flip",
            clue: "flip",
            blurb: "50% chance of a Heads or Tails",
            help: vec![
                "Returns one of the two strings 'Heads' or 'Tails'. There is an equal probability",
                "of returning either one.",
            ],
        },
        help::Hint {
            cmd: "coin",
            clue: "coin",
            blurb: "alias for flip",
            help: vec![
                "Returns one of the two strings 'Heads' or 'Tails'. There is an equal probability",
                "of returning either one.",
            ],
        },
    ]
}

#[cfg(test)]
mod tests
{
    const NUM_TRIES: u32 = 3;

    #[test]
    fn coin_tosses()
    {
        let expected = ["Heads", "Tails"];

        for _ in 1..=NUM_TRIES
        {
            let flip = super::flip();
            assert_ne!(expected.iter().find(|&&x| x == flip), None);
        }
    }
}
