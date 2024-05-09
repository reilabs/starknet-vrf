mod curve;
mod hash;

// cipher suite string
pub const STARK_PEDERSEN_SSWU: u8 = 0xff;

#[cfg(test)]
mod tests {
    use ark_ec::{
        hashing::{curve_maps::swu::SWUMap, map_to_curve_hasher::MapToCurve},
        short_weierstrass::{Affine, SWCurveConfig},
        CurveConfig, CurveGroup,
    };
    use ark_ff::{BigInt, MontFp};
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
    use starknet_crypto::pedersen_hash;
    use starknet_ff::FieldElement;

    use crate::{
        curve::{Fr, ScalarField, StarkCurve},
        STARK_PEDERSEN_SSWU,
    };

    pub type ECVRFProof = (Affine<StarkCurve>, ScalarField, ScalarField);

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
        let proof = prove(&public_key, &secret_key, alpha);
        let beta = proof_to_hash(&proof);
        verify(&public_key, alpha, &proof);
        println!("proof verified, beta = {beta}");
    }

    fn pedersen_hash_str(message: &[u8]) -> Fr {
        let mut h = pedersen_hash(
            &FieldElement::from_byte_slice_be(&[message[0]]).unwrap(),
            &FieldElement::from_byte_slice_be(&[message[1]]).unwrap(),
        );
        for input in &message[2..message.len()] {
            h = pedersen_hash(&h, &FieldElement::from_byte_slice_be(&[*input]).unwrap());
        }

        Fr::from(BigInt::new(h.into_mont()))
    }

    fn hash_to_curve(pk: &Affine<StarkCurve>, message: &[u8]) -> Affine<StarkCurve> {
        let mut pk_string = Vec::new();
        pk.serialize_compressed(&mut pk_string).unwrap();
        let t_string = [&[STARK_PEDERSEN_SSWU, 0x01], pk_string.as_slice(), message].concat();
        let t: Fr = pedersen_hash_str(&t_string);
        let curve_mapper = SWUMap::<StarkCurve>::new().unwrap();
        curve_mapper.map_to_curve(t).unwrap()
    }

    fn nonce(secret_key: &ScalarField, message: &[u8]) -> ScalarField {
        // TODO: implement an actual nonce
        MontFp!("1")
    }

    fn hash_points(points: &[Affine<StarkCurve>]) -> ScalarField {
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

    fn prove(
        public_key: &Affine<StarkCurve>,
        secret_key: &ScalarField,
        alpha: &[u8],
    ) -> ECVRFProof {
        let h = hash_to_curve(public_key, alpha);
        let mut h_string = Vec::new();
        h.serialize_compressed(&mut h_string).unwrap();

        let gamma: Affine<StarkCurve> = (h * secret_key).into();
        let k = nonce(&secret_key, &h_string);
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

    fn proof_to_hash(proof: &ECVRFProof) -> Fr {
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

    fn verify(pk: &Affine<StarkCurve>, alpha: &[u8], proof: &ECVRFProof) {
        let (gamma, c, s) = proof;
        let h = hash_to_curve(pk, alpha);
        let u = (StarkCurve::GENERATOR * s) - (*pk * *c);
        let v = (h * s) - (*gamma * *c);
        let c_prim = hash_points(&[*pk, h, *gamma, u.into(), v.into()]);
        assert_eq!(*c, c_prim);
    }
}
