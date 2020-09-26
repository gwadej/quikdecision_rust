extern crate rand;
extern crate regex;

use std::fmt;
use rand::Rng;
use rand::seq::SliceRandom;

pub mod coin;
pub mod deck;
pub mod dice;
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
    RollDice(Vec<dice::Roll>),
    Selection(Vec<String>),
    Shuffle(Vec<String>),
    Oracle(oracle::Oracle),
}

/// Structure containing the documentation for a quik decision command
#[derive(Debug)]
pub struct ApiDoc
{
    pub name: &'static str,
    pub params: Vec<&'static str>,
    pub hint: &'static str,
    pub help: Vec<&'static str>,
}

/// The Decision enum encapsulates values returned from the decide method.
#[derive(Debug)]
pub enum Decision
{
    Text(String),
    LabelledText{ value: String, label: String },
    Num(i32),
    AnnotatedNum{ value: u32, extra: String },
    Bool(bool),
    List(Vec<String>),
    Card(deck::Card),
}

impl fmt::Display for Decision
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Decision::Text(v) => write!(f, "'{}'", v),
            Decision::LabelledText{value, ..} => write!(f, "'{}'", value),
            Decision::Num(v) => write!(f, "{}", v),
            Decision::AnnotatedNum{value, extra} => write!(f, "{}: '{}'", value, extra),
            Decision::Bool(v) => write!(f, "{}", v),
            Decision::List(v) => write!(f, "'{}'", v.join(", ")),
            Decision::Card(v) => write!(f, "{}", v.to_string()),
        }
    }
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
        match *self
        {
            Command::CoinFlip(ref coin)      => coin.decide(),
            Command::DrawCard(ref deck)      => deck.decide(),
            Command::PickNumber(ref range)   => range.decide(),
            Command::PercentTrue(ref likely) => likely.decide(),
            Command::RollDice(ref expr)      => dice::roll(expr),
            Command::Selection(ref strvec)   => select::choose(strvec),
            Command::Shuffle(ref strvec)     => shuffle::order(strvec),
            Command::Oracle(ref oracle)      => oracle.decide(),
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
            (Command::CoinFlip(_),        Command::CoinFlip(_)) => true,
            (Command::DrawCard(dl),       Command::DrawCard(dr)) => dl == dr,
            (Command::Oracle(_),          Command::Oracle(_)) => true,
            (Command::PickNumber(rl),     Command::PickNumber(rr)) => rl == rr,
            (Command::PercentTrue(sp),    Command::PercentTrue(op)) => sp == op,
            (Command::RollDice(sdice),    Command::RollDice(odice)) => sdice == odice,
            (Command::Selection(sstrs),   Command::Selection(ostrs)) => sstrs == ostrs,
            (Command::Shuffle(sstrs),     Command::Shuffle(ostrs)) => sstrs == ostrs,
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
