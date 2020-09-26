use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    UnrecognizedDeck,
    NotANumber,
    DiceMissingExpr,
    DiceBadExpr,
    DiceBadSides,
    DiceBadType,
    UnparseableTerm,
    PercentZero,
    PercentOverflow,
    EmptyRange,
    ListEmpty,
    ListOne,
}

impl From<Error> for String {
    fn from(error: Error) -> Self {
        use Error::*;
        match error
        {
            UnrecognizedDeck => "Unrecognized deck type".to_owned(),
            NotANumber       => "Non-number somehow passed parsing".to_owned(),
            DiceMissingExpr  => "Missing dice expression".to_owned(),
            DiceBadExpr      => "Failed parsing dice expression".to_owned(),
            DiceBadSides     => "No sides specified".to_owned(),
            DiceBadType      => "Unrecognized die type".to_owned(),
            UnparseableTerm  => "Unparseable term".to_owned(),
            PercentZero      => "percent arg cannot be 0".to_owned(),
            PercentOverflow  => "percent arg cannot be 100 percent or greater".to_owned(),
            EmptyRange       => "High parameter cannot equal low parameter".to_owned(),
            ListEmpty        => "Missing required strings".to_owned(),
            ListOne          => "Must supply at least two strings".to_owned(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", String::from(*self))
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        use Error::*;
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
