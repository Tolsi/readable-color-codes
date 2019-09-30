use super::{DecodeError, encoder, decoder::Decoder};

pub trait Alphabet<T> {
    fn encode(self, input: &[u8]) -> T;

    fn decode(self, input: &T) -> Result<Vec<u8>, DecodeError>;
}

impl<'a> Alphabet<Vec<u32>> for &[u32] {
    #[inline(always)]
    fn encode(self, input: &[u8]) -> Vec<u32> {
        let out = encoder::encode(self, input);
        out.iter().rev().map(|i| *i).collect()
    }

    #[inline(always)]
    fn decode(self, input: &Vec<u32>) -> Result<Vec<u8>, DecodeError> {
        Decoder { 0: self }.decode(&input.as_slice())
    }
}
