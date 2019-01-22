use crate::{Command, Decision, Decider};

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
    pub fn iter(self) -> QdIter
    {
        QdIter::new(self)
    }
}
