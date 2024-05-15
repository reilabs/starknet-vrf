mod curve;
mod ecvrf;
pub mod error;
pub mod hash;

pub use ecvrf::*;

#[cfg(test)]
mod tests {
    use ark_ec::{
        short_weierstrass::{Affine, SWCurveConfig},
        CurveGroup,
    };
    use ark_ec::hashing::map_to_curve_hasher::MapToCurve;
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
    use starknet_ff::FieldElement;
    use ark_ff::{BigInt, PrimeField};

    use crate::{
        curve::{BaseField, ScalarField, StarkCurve},
        hash::PedersenHash,
        ECVRF, STARK_PEDERSEN_SSWU,
    };

    #[test]
    fn point_serialization() {
        let mut buf = Vec::new();

        let g = StarkCurve::GENERATOR;
        assert!(g.is_on_curve());
        assert!(g.is_in_correct_subgroup_assuming_on_curve());

        g.serialize_compressed(&mut buf).unwrap();
        let deg = Affine::<StarkCurve>::deserialize_compressed(&*buf).unwrap();
        assert_eq!(g, deg);
    }

    #[test]
    fn it_matches_cairo_hashing() {
        use starknet_crypto::poseidon_hash_many;

        let buf = [
            FieldElement::from_dec_str(
                "874739451078007766457464989774322083649278607533249481151382481072868806602",
            )
            .unwrap(),
            FieldElement::from_dec_str(
                "152666792071518830868575557812948353041420400780739481342941381225525861407",
            )
            .unwrap(),
            FieldElement::from_dec_str("1").unwrap(),
            FieldElement::from_dec_str("42").unwrap(),
        ];

        let secret_key = ScalarField::from(190);
        let public_key = (StarkCurve::GENERATOR * secret_key).into_affine();
        let ecvrf =
        ECVRF::<StarkCurve, PedersenHash>::new(STARK_PEDERSEN_SSWU, public_key).unwrap();

        let hash = poseidon_hash_many(buf.as_slice());
        let hash_in_base = BaseField::new_unchecked(BigInt(hash.into_mont()));

        let point = ecvrf.mapper.map_to_curve(hash_in_base).unwrap();
        println!("hash {hash} {hash_in_base}");
        println!("point {point}");
    }

    #[test]
    fn it_proves() {
        let secret_key = ScalarField::from(190);
        let public_key = (StarkCurve::GENERATOR * secret_key).into_affine();

        let alpha = b"test";
        let ecvrf =
            ECVRF::<StarkCurve, PedersenHash>::new(STARK_PEDERSEN_SSWU, public_key).unwrap();
        let proof = ecvrf.prove(&secret_key, alpha).unwrap();
        let beta = ecvrf.proof_to_hash(&proof).unwrap();
        ecvrf.verify(alpha, &proof).expect("proof correct");
        println!("proof verified, beta = {beta}");
    }
}
