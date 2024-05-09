use ark_ec::{
    hashing::{curve_maps::swu::SWUMap, map_to_curve_hasher::MapToCurve},
    short_weierstrass::{Affine, SWCurveConfig},
    CurveConfig,
};
use ark_ff::BigInt;
use ark_serialize::CanonicalSerialize;
use starknet_crypto::pedersen_hash;
use starknet_ff::FieldElement;

use crate::curve::{BaseField, ScalarField, StarkCurve};

pub const STARK_PEDERSEN_SSWU: u8 = 0xff;

pub type ECVRFProof = (
    Affine<StarkCurve>,
    <StarkCurve as CurveConfig>::ScalarField,
    <StarkCurve as CurveConfig>::ScalarField,
);

pub fn pedersen_hash_str(message: &[u8]) -> BaseField {
    let mut h = pedersen_hash(
        &FieldElement::from_byte_slice_be(&[message[0]]).unwrap(),
        &FieldElement::from_byte_slice_be(&[message[1]]).unwrap(),
    );
    for input in &message[2..message.len()] {
        h = pedersen_hash(&h, &FieldElement::from_byte_slice_be(&[*input]).unwrap());
    }

    BaseField::from(BigInt::new(h.into_mont()))
}

pub fn hash_to_curve(pk: &Affine<StarkCurve>, message: &[u8]) -> Affine<StarkCurve> {
    let mut pk_string = Vec::new();
    pk.serialize_compressed(&mut pk_string).unwrap();
    let t_string = [&[STARK_PEDERSEN_SSWU, 0x01], pk_string.as_slice(), message].concat();
    let t: BaseField = pedersen_hash_str(&t_string);
    let curve_mapper = SWUMap::<StarkCurve>::new().unwrap();
    curve_mapper.map_to_curve(t).unwrap()
}

// 5.4.2.2. ECVRF Nonce Generation from RFC 8032
pub fn nonce(secret_key: &ScalarField, message: &[u8]) -> ScalarField {
    let mut sk_string = Vec::new();
    secret_key.serialize_compressed(&mut sk_string).unwrap();

    let fr = pedersen_hash_str(&[sk_string.as_slice(), message].concat());
    ScalarField::from(fr.0)
}

pub fn hash_points(points: &[Affine<StarkCurve>]) -> ScalarField {
    let mut string = vec![STARK_PEDERSEN_SSWU, 0x02];
    for point in points {
        let mut buf = Vec::new();
        point.serialize_compressed(&mut buf).unwrap();
        string.extend_from_slice(&buf);
    }
    string.extend_from_slice(&[0x00]);

    // typically challenges have half the number of bits of the
    // scalar field.
    // for now this returns a full scalar field value
    let fr = pedersen_hash_str(&string);
    ScalarField::from(fr.0)
}

pub fn prove(
    public_key: &Affine<StarkCurve>,
    secret_key: &ScalarField,
    alpha: &[u8],
) -> ECVRFProof {
    let h = hash_to_curve(public_key, alpha);
    let mut h_string = Vec::new();
    h.serialize_compressed(&mut h_string).unwrap();

    let gamma: Affine<StarkCurve> = (h * secret_key).into();
    let k = nonce(secret_key, &h_string);
    let c = hash_points(&[
        *public_key,
        h,
        gamma,
        (StarkCurve::GENERATOR * k).into(),
        (h * k).into(),
    ]);
    let s = k + c * secret_key;
    (gamma, c, s)
}

pub fn proof_to_hash(proof: &ECVRFProof) -> BaseField {
    let mut string = vec![STARK_PEDERSEN_SSWU, 0x03];

    let mut cofactor_buf: [u64; 4] = [0; 4];
    for (i, limb) in StarkCurve::COFACTOR.iter().enumerate() {
        cofactor_buf[i] = *limb;
    }
    let cofactor_gamma = proof.0 * ScalarField::from(BigInt::new(cofactor_buf));
    // our cofactor is 1
    assert_eq!(proof.0, cofactor_gamma);

    let mut buf = Vec::new();
    proof.0.serialize_compressed(&mut buf).unwrap();

    string.extend_from_slice(&buf);
    string.extend_from_slice(&[0x00]);

    pedersen_hash_str(&string)
}

pub fn verify(pk: &Affine<StarkCurve>, alpha: &[u8], proof: &ECVRFProof) {
    let (gamma, c, s) = proof;
    let h = hash_to_curve(pk, alpha);
    let u = (StarkCurve::GENERATOR * s) - (*pk * *c);
    let v = (h * s) - (*gamma * *c);
    let c_prim = hash_points(&[*pk, h, *gamma, u.into(), v.into()]);
    assert_eq!(*c, c_prim);
}
