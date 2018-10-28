use super::Command;

const COIN_SIDES: [&str; 2] = [ "Heads", "Tails" ];

pub fn command() -> Result<Command, String>
{
    Ok(Command::CoinFlip)
}

pub fn flip() -> String
{
    String::from(super::random_choice(&COIN_SIDES))
}
