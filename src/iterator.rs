use crate::{Command, Decision, Decider};

#[derive(Debug, PartialEq)]
pub struct QdIter<'a>
{
    decider: &'a Command,
}

impl<'a> QdIter<'a>
{
    fn new(decider: &Command) -> QdIter
    {
        QdIter { decider }
    }
}

impl<'a> Iterator for QdIter<'a>
{
    type Item = Decision;

    fn next(&mut self) -> Option<Decision>
    {
        Some(self.decider.decide())
    }
}

impl Command
{
    /// Convert the Command into an infinite iterator that simplifies
    /// calling it multiple times.
    pub fn iter(&self) -> QdIter
    {
        QdIter::new(&self)
    }
}

#[cfg(test)]
mod tests
{
    use spectral::prelude::*;

    use super::*;
    use crate::{Command, Decision};

    use crate::coin;

    #[test]
    fn test_create()
    {
        let cmd = Command::CoinFlip(coin::Coin{});
        let ocmd = Command::CoinFlip(coin::Coin{});
        assert_that!(cmd.iter()).is_equal_to(&QdIter::new(&ocmd));
    }

    #[test]
    fn test_iter()
    {
        let mut it = Command::CoinFlip(coin::Coin{}).iter();
        match it.next()
        {
            Some(Decision::Text(_)) => assert!(true, "Correct type"),
            Some(_) => assert!(false, "Wrong Decision type"),
            None => assert!(false, "No value returned"),
        }
    }
}
