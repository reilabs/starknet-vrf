use ark_ff::BigInt;
use starknet_crypto::pedersen_hash;
use starknet_ff::FieldElement;

pub trait HashToField {
    fn new() -> Self;
    fn hash(&self, msg: &[u8]) -> BigInt<4>;
}

pub struct PedersenHash;

impl HashToField for PedersenHash {
    fn new() -> Self {
        Self
    }

    fn hash(&self, msg: &[u8]) -> BigInt<4> {
        pedersen_hash_str(msg)
    }
}

fn pedersen_hash_str(message: &[u8]) -> BigInt<4> {
    let mut h = pedersen_hash(
        &FieldElement::from_byte_slice_be(&[message[0]]).unwrap(),
        &FieldElement::from_byte_slice_be(&[message[1]]).unwrap(),
    );
    for input in &message[2..message.len()] {
        h = pedersen_hash(&h, &FieldElement::from_byte_slice_be(&[*input]).unwrap());
    }

    BigInt::new(h.into_mont())
}
