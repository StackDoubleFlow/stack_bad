use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub line: usize,
    pub col: usize,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "stack bad")
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
