use super::bigint::BigUint;
use super::DecodeError;

pub(crate) struct Decoder<'a>(pub &'a [u32]);

impl<'a> Decoder<'a> {
    pub fn decode(&self, input: &'a &[u32]) -> Result<Vec<u8>, DecodeError> {
        if input.is_empty() {
            return Ok(Vec::new());
        }
        // todo use range
        let alpha = self.0;
        let base = alpha.len() as u32;

        let mut big = BigUint::with_capacity(4);

        for c in input.iter() {
            let r: Option<&u32> = Some(c);
            if let Some(carry) = r {
                big.mul_add(base, *carry);
            } else {
                return Err(DecodeError);
            }
        }

        let mut bytes = big.into_bytes_be();

        let leader = alpha[0];

        let leaders = input.iter().take_while(|byte| **byte == leader).count();

        for _ in 0..leaders {
            bytes.insert(0, 0);
        }

        Ok(bytes)
    }

}