use std::fmt;
use crate::deck;

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
