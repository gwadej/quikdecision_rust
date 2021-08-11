use derive_more::{Display, IsVariant};
use crate::deck;

/// The Decision enum encapsulates values returned from the decide method.
#[derive(Debug, IsVariant, Display)]
pub enum Decision
{
    #[display(fmt = "'{}'", _0)]
    Text(String),
    #[display(fmt = "'{}'", value)]
    LabelledText{ value: String, label: String },
    #[display(fmt = "'{}'", _0)]
    Num(i32),
    #[display(fmt = "{}: '{}'", value, extra)]
    AnnotatedNum{ value: u32, extra: String },
    #[display(fmt = "{}", _0)]
    Bool(bool),
    #[display(fmt = "'{:?}'", _0)]
    List(Vec<String>),
    #[display(fmt = "{}", _0)]
    Card(deck::Card),
}
