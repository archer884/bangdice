use std::error::Error;
use std::fmt::{self, Display};
use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseExpressionError {
    TooManySegments,
    Int(ParseIntError),
    Empty,
}

impl Display for ParseExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseExpressionError::TooManySegments => {
                f.write_str("Expression has too many segments")
            }
            ParseExpressionError::Int(e) => write!(f, "{}", e),
            ParseExpressionError::Empty => f.write_str("Empty expression"),
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
