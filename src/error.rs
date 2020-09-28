use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum QuikError {
    #[error("Unrecognized deck type")]
    UnrecognizedDeck,
    #[error("Non-number somehow passed parsing")]
    NotANumber,
    #[error("Missing dice expression")]
    DiceMissingExpr,
    #[error("Failed parsing dice expression")]
    DiceBadExpr,
    #[error("No sides specified")]
    DiceBadSides,
    #[error("Unrecognized die type")]
    DiceBadType,
    #[error("Unparseable term")]
    UnparseableTerm,
    #[error("percent arg cannot be 0")]
    PercentZero,
    #[error("percent arg cannot be 100 percent or greater")]
    PercentOverflow,
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

impl PartialEq for QuikError {
    fn eq(&self, other: &Self) -> bool {
        use QuikError::*;
        match (self, other)
        {
            (UnrecognizedDeck, UnrecognizedDeck) => true,
            (NotANumber, NotANumber)             => true,
            (DiceMissingExpr, DiceMissingExpr)   => true,
            (DiceBadExpr, DiceBadExpr)           => true,
            (DiceBadSides, DiceBadSides)         => true,
            (DiceBadType, DiceBadType)           => true,
            (UnparseableTerm, UnparseableTerm)   => true,
            (PercentZero, PercentZero)           => true,
            (PercentOverflow, PercentOverflow)   => true,
            (EmptyRange, EmptyRange)             => true,
            (ListEmpty, ListEmpty)               => true,
            (ListOne, ListOne)                   => true,
            (_, _)                               => false,
        }
    }
}
