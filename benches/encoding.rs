#[macro_use]
extern crate bencher;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate readable_color_codes;

use bencher::Bencher;

use readable_color_codes::encoding::{Alphabet, decode, encode};

lazy_static! {
    static ref base2: Vec<u32> = {
        "01".chars().map(|c| c as u32).collect()
    };
    static ref base16: Vec<u32> = {
        "0123456789abcdef".chars().map(|c| c as u32).collect()
        };
    static ref base58: Vec<u32> = {
        "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".chars().map(|c| c as u32).collect()
    };
}

fn random_input(size: usize) -> Vec<u8> {
    let mut v = vec![0; size];

    for x in v.iter_mut() {
        *x = rand::random()
    }

    v
}

fn test_decode<T, A: Alphabet<T> + Copy>(bench: &mut Bencher, alph: A) {
    let input = random_input(100);
    let out = encode(alph, &input);

    bench.iter(|| decode(alph, &out).unwrap());
}

fn test_encode<T, A: Alphabet<T> + Copy>(bench: &mut Bencher, alph: A) {
    let input = random_input(100);

    bench.iter(|| encode(alph, &input));
}

// Actual benchmarks

// Encode UTF-8
fn encode_base2(bench: &mut Bencher) {
    test_encode(bench, base2.as_slice());
}

fn encode_base16(bench: &mut Bencher) {
    test_encode(bench, base16.as_slice());
}

fn encode_base58(bench: &mut Bencher) {
    test_encode(bench, base58.as_slice());
}

// Decode UTF-8
fn decode_base2(bench: &mut Bencher) {
    test_decode(bench, base2.as_slice());
}

fn decode_base16(bench: &mut Bencher) {
    test_decode(bench, base16.as_slice());
}

fn decode_base58(bench: &mut Bencher) {
    test_decode(bench, base58.as_slice());
}

benchmark_group!(
    benches,
    encode_base2,
    decode_base2,
    encode_base16,
    decode_base16,
    encode_base58,
    decode_base58
);
benchmark_main!(benches);
