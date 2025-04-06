use core::error::Error;
use core::result::Result;
use core::fmt;

#[derive(Debug)]
pub struct RatError {}

impl fmt::Display for RatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "todo")
    }
}

impl Error for RatError {}
pub type RatResult<T> = Result<T, RatError>;
