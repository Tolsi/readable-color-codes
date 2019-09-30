extern crate itertools;

use itertools::*;

mod encoding;

fn main() {
    let alph: Vec<u32> = (('\u{0}' as u32)..(2_i32.pow(24) as u32)).collect();
    let bytes: Vec<u8> = (0..32).collect();
    let encoded = encoding::encode(alph.as_slice(), bytes.as_slice());
    let decoded = encoding::decode(alph.as_slice(), &encoded).unwrap();
    println!("[{}]{:?}", encoded.len(), encoded);
    println!("[{}]{:?}", decoded.len(), decoded);
}