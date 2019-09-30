pub mod alphabet;
mod bigint;
pub mod decoder;
pub mod encoder;

pub use alphabet::Alphabet;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DecodeError;

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to decode the given data")
    }
}

impl Error for DecodeError {
    fn description(&self) -> &str {
        "Can not decode the provided data"
    }
}

/// Encode an input vector using the given alphabet.
pub fn encode<T, A: Alphabet<T>>(alphabet: A, input: &[u8]) -> T {
    alphabet.encode(input)
}

/// Decode an input vector using the given alphabet.
pub fn decode<T, A: Alphabet<T>>(alphabet: A, input: &T) -> Result<Vec<u8>, DecodeError> {
    alphabet.decode(input)
}