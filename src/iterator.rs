use crate::{Command, Decision, Decider};

#[derive(Debug)]
pub struct QdIter
{
    decider: Command,
}

impl QdIter
{
    fn new(decider: Command) -> QdIter
    {
        QdIter { decider }
    }
}

impl Iterator for QdIter
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
    pub fn iter(self) -> QdIter
    {
        QdIter::new(self)
    }
}

impl PartialEq for QdIter
{
    fn eq(&self, other: &QdIter) -> bool
    {
        self.decider == other.decider
    }
}

#[cfg(test)]
mod tests
{
    use spectral::prelude::*;

    use super::*;
    use crate::{Command, Decision};

    #[test]
    fn test_create()
    {
        let cmd = Command::CoinFlip;
        let ocmd = Command::CoinFlip;
        assert_that!(cmd.iter()).is_equal_to(&QdIter::new(ocmd));
    }

    #[test]
    fn test_iter()
    {
        let mut it = Command::CoinFlip.iter();
        match it.next()
        {
            Some(Decision::Text(_)) => assert!(true, "Correct type"),
            Some(_) => assert!(false, "Wrong Decision type"),
            None => assert!(false, "No value returned"),
        }
    }
}
