extern crate derive_more;
extern crate rand;
extern crate regex;
extern crate thiserror;

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
#[derive(Debug, PartialEq)]
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

pub type Result<T> = std::result::Result<T,error::QuikError>;
pub type Error = error::QuikError;
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

/// DecisionAssertions trait to support spectral tests on the Decision enum.
trait DecisionAssertions<'s>
{
    /// Returns true if the Decision being tested matches the correct variant.
    fn is_text_decision(&self) -> bool;
    fn is_labelled_text_decision(&self) -> bool;
    fn is_num_decision(&self) -> bool;
    fn is_annotated_num_decision(&self) -> bool;
    fn is_bool_decision(&self) -> bool;
    fn is_list_decision(&self) -> bool;
    fn is_card_decision(&self) -> bool;
}

#[cfg(test)]
impl<'s> DecisionAssertions<'s> for spectral::Spec<'s, Decision>
{
    fn is_text_decision(&self) -> bool { self.subject.is_text() }
    fn is_labelled_text_decision(&self) -> bool { self.subject.is_labelled_text() }
    fn is_num_decision(&self) -> bool { self.subject.is_num() }
    fn is_annotated_num_decision(&self) -> bool { self.subject.is_num() }
    fn is_bool_decision(&self) -> bool { self.subject.is_bool() }
    fn is_list_decision(&self) -> bool { self.subject.is_list() }
    fn is_card_decision(&self) -> bool { self.subject.is_card() }
}
