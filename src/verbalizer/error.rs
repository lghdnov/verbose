use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum VerbalizeError {
    NumberTooLarge(u64, u64),
    Fmt(std::fmt::Error),
}

impl From<std::fmt::Error> for VerbalizeError {
    fn from(err: std::fmt::Error) -> Self {
        Self::Fmt(err)
    }
}

impl fmt::Display for VerbalizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VerbalizeError::NumberTooLarge(n, m) => {
                write!(f, "Number {} exceeds maximum supported value {}", n, m)
            }
            VerbalizeError::Fmt(err) => write!(f, "Fmt error: {}", err),
        }
    }
}

impl std::error::Error for VerbalizeError {}
