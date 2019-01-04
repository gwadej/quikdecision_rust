extern crate rand;
extern crate regex;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub mod coin;
pub mod dice;
pub mod oracle;
pub mod percent;
pub mod pick;
pub mod select;
pub mod shuffle;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Enum defining the types of quik decision commands, and the parameters that
/// determine their functioning.
#[derive(Debug)]
pub enum Command
{
    CoinFlip,
    PickNumber(i32, i32),
    PercentTrue(u32),
    RollDice(Vec<dice::Roll>),
    Selection(Vec<String>),
    Shuffle(Vec<String>),
    Oracle,
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
}

/// trait for making a random decision.
pub trait Decider
{
    fn decide(self) -> Decision;
}

/// Trait implementation for making a random decision for a Command.
impl Decider for Command
{
    /// Perform appropriate command returning a Decision object.
    fn decide(self) -> Decision
    {
        match self
        {
            Command::CoinFlip => coin::flip(),
            Command::PickNumber(low, high) => pick::choose(low, high),
            Command::PercentTrue(likely) => percent::choose(likely),
            Command::RollDice(expr) => dice::roll(expr),
            Command::Selection(strvec) => select::choose(strvec),
            Command::Shuffle(strvec) => shuffle::order(strvec),
            Command::Oracle => oracle::choose(),
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
pub fn pick_one<T>(choices: &[T]) -> String
    where T : std::string::ToString + std::clone::Clone
{
    let mut rng = thread_rng();
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
            (Command::CoinFlip,           Command::CoinFlip) => true,
            (Command::Oracle,             Command::Oracle) => true,
            (Command::PickNumber(sl, sh), Command::PickNumber(ol, oh)) => sl == ol && sh == oh,
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
            (Decision::Text(_),         Decision::Text(_)) => true,
            (Decision::LabelledText{value:_, label:_},
             Decision::LabelledText{value:_, label:_}) => true,
            (Decision::Num(_),          Decision::Num(_)) => true,
            (Decision::AnnotatedNum{value:_, extra:_},
             Decision::AnnotatedNum{value:_, extra:_}) => true,
            (Decision::Bool(_),         Decision::Bool(_)) => true,
            (Decision::List(_),         Decision::List(_)) => true,
            (_, _) => false,
        }
    }
}
