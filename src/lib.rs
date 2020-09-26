extern crate rand;
extern crate regex;

use rand::Rng;
use rand::seq::SliceRandom;

pub mod coin;
pub mod deck;
pub mod decision;
pub mod dice;
pub mod error;
pub mod oracle;
pub mod percent;
pub mod pick;
pub mod select;
pub mod shuffle;
pub mod iterator;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Enum defining the types of quik decision commands, and the parameters that
/// determine their functioning.
#[derive(Debug)]
pub enum Command
{
    CoinFlip(coin::Coin),
    DrawCard(deck::Deck),
    PickNumber(pick::Picker),
    PercentTrue(percent::Likely),
    RollDice(dice::Expr),
    Selection(select::Choices),
    Shuffle(shuffle::Choices),
    Oracle(oracle::Oracle),
}

pub type Result<T> = std::result::Result<T,error::Error>;
pub type Error = error::Error;
pub type Decision = decision::Decision;

/// Structure containing the documentation for a quik decision command
#[derive(Debug)]
pub struct ApiDoc
{
    pub name: &'static str,
    pub params: Vec<&'static str>,
    pub hint: &'static str,
    pub help: Vec<&'static str>,
}

/// trait for making a random decision.
pub trait Decider
{
    fn decide(&self) -> Decision;
}

/// Trait implementation for making a random decision for a Command.
impl Decider for Command
{
    /// Perform appropriate command returning a Decision object.
    fn decide(&self) -> Decision
    {
        self.get_decider().decide()
    }
}

impl Command {
    fn get_decider(&self) -> &dyn Decider {
        match self
        {
            Command::CoinFlip(coin)      => coin,
            Command::DrawCard(deck)      => deck,
            Command::PickNumber(range)   => range,
            Command::PercentTrue(likely) => likely,
            Command::RollDice(expr)      => expr,
            Command::Selection(choices)  => choices,
            Command::Shuffle(choices)    => choices,
            Command::Oracle(oracle)      => oracle,
        }
    }
}

/// Return the library version as a string.
pub fn version() -> &'static str
{
    VERSION
}

/// Randomly select one of the supplied choices and return it as a String.
///
/// choices:  an array slice of a type that can be cloned and converted to a
///           String.
pub fn pick_one<R, T>(mut rng: &mut R, choices: &[T]) -> String
    where R: Rng,
          T: std::string::ToString + std::clone::Clone
{
    choices.choose(&mut rng).expect("Somehow ended up with no strings").to_string()
}

#[cfg(test)]
extern crate spectral;

/// Add PartialEq implementation for Command for use only in tests.
impl PartialEq for Command
{
    fn eq(&self, other: &Command) -> bool
    {
        match (self, other)
        {
            (Command::CoinFlip(_),     Command::CoinFlip(_)) => true,
            (Command::DrawCard(dl),    Command::DrawCard(dr)) => dl == dr,
            (Command::Oracle(_),       Command::Oracle(_)) => true,
            (Command::PickNumber(rl),  Command::PickNumber(rr)) => rl == rr,
            (Command::PercentTrue(sp), Command::PercentTrue(op)) => sp == op,
            (Command::RollDice(el),    Command::RollDice(er)) => el == er,
            (Command::Selection(cl),   Command::Selection(cr)) => cl == cr,
            (Command::Shuffle(cl),     Command::Shuffle(cr)) => cl == cr,
            (_, _) => false,
        }
    }
}

/// DecisionAssertions trait to support spectral tests on the Decision enum.
trait DecisionAssertions<'s>
{
    /// Returns true if the Decision being tested matches the same variant as the
    /// supplied other.
    fn matches_enum_variant(&self, other: Decision) -> bool;
}

impl<'s> DecisionAssertions<'s> for spectral::Spec<'s, Decision>
{
    fn matches_enum_variant(&self, other: Decision) -> bool
    {
        match (self.subject, other)
        {
            (Decision::Text(_),          Decision::Text(_)) => true,
            (Decision::LabelledText{..}, Decision::LabelledText{..}) => true,
            (Decision::Num(_),           Decision::Num(_)) => true,
            (Decision::AnnotatedNum{..}, Decision::AnnotatedNum{..}) => true,
            (Decision::Bool(_),          Decision::Bool(_)) => true,
            (Decision::List(_),          Decision::List(_)) => true,
            (Decision::Card(_),          Decision::Card(_)) => true,
            (_, _) => false,
        }
    }
}
