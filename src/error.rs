use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum QuikError {
    #[error("Unrecognized deck type {0}")]
    UnrecognizedDeck(String),
    #[error("Non-number somehow passed parsing {0}")]
    NotANumber(String),
    #[error("Missing dice expression")]
    DiceMissingExpr,
    #[error("Failed parsing dice expression {0}")]
    DiceBadExpr(String),
    #[error("No sides specified")]
    DiceBadSides,
    #[error("Unrecognized die type {0}")]
    DiceBadType(String),
    #[error("Unparseable term")]
    UnparseableTerm,
    #[error("percent arg cannot be 0")]
    PercentZero,
    #[error("percent arg cannot be 100 percent or greater {0}")]
    PercentOverflow(u32),
    #[error("High parameter cannot equal low parameter")]
    EmptyRange,
    #[error("Missing required strings")]
    ListEmpty,
    #[error("Must supply at least two strings")]
    ListOne,
}

impl From<QuikError> for String {
    fn from(error: QuikError) -> Self {
        format!("{}", error)
    }
}
