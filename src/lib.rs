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
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

    use crate::{
        curve::{ScalarField, StarkCurve},
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
