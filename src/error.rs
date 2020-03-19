use std::error::Error;
use std::fmt::{self, Display};
use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseExpressionError {
    /// Empty dice expression.
    Empty,

    /// Indicates a failure to parse an integer.
    Int(ParseIntError),
    
    /// Thrown in the event someone tries to roll a one-sided die.
    Invalid,

    /// Malformed dice expression.
    TooManySegments,
}

impl Display for ParseExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseExpressionError::Empty => f.write_str("Empty expression"),
            ParseExpressionError::Int(e) => write!(f, "{}", e),
            ParseExpressionError::Invalid => f.write_str("Huh?"),
            ParseExpressionError::TooManySegments => {
                f.write_str("Expression has too many segments")
            }
        }
    }
}

impl From<ParseIntError> for ParseExpressionError {
    fn from(e: ParseIntError) -> Self {
        ParseExpressionError::Int(e)
    }
}

impl Error for ParseExpressionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParseExpressionError::Int(e) => Some(e),
            _ => None,
        }
    }
}
