mod curve;
mod ecvrf;
pub mod error;

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
        ecvrf::{proof_to_hash, prove, verify},
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
        let proof = prove(&public_key, &secret_key, alpha).unwrap();
        let beta = proof_to_hash(&proof).unwrap();
        verify(&public_key, alpha, &proof).expect("proof correct");
        println!("proof verified, beta = {beta}");
    }
}
