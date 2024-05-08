use ark_ec::{
    hashing::{
        curve_maps::swu::{SWUConfig, SWUMap},
        map_to_curve_hasher::MapToCurveBasedHasher,
        HashToCurve,
    },
    short_weierstrass::{Affine, Projective, SWCurveConfig},
    CurveConfig,
};
use ark_ff::{
    field_hashers::DefaultFieldHasher, Fp, Fp256, Fp64, MontBackend, MontConfig, MontFp, PrimeField,
};
use curve::TestSWUMapToCurveConfig;
use hash::PedersenHash;
use sha2::{
    digest::{
        generic_array::ArrayLength, DynDigest, FixedOutput, FixedOutputReset, OutputSizeUser,
    },
    Sha256,
};
use starknet_crypto::FieldElement;
use starknet_curve::{
    curve_params::{ALPHA, BETA, GENERATOR},
    AffinePoint, ProjectivePoint,
};

mod curve;
mod hash;

// cipher suite string
pub const STARK_SHA256_TAI: u8 = 0xff;

/// Different errors that can be raised when proving/verifying VRFs
#[derive(Debug)]
pub enum Error {}

// assume only one ciphersuite for now?
pub struct ECVRF {}

#[inline(always)]
fn mul_by_bits(x: &AffinePoint, y: &FieldElement) -> AffinePoint {
    let x = ProjectivePoint::from_affine_point(x);
    let y = y.to_bits_le();
    let z = &x * &y;
    AffinePoint::from(&z)
}

fn EC2OSP(point: &AffinePoint, compressed: bool) -> Vec<u8> {
    if point.infinity {
        return vec![0];
    }

    if compressed {
        let x = point.x.to_bytes_be();
        let y = 2 + (point.y.to_bytes_be()[31] % 2);

        [&[y], x.as_slice()].concat()
    } else {
        todo!()
    }
}

// fn OS2IP(msg: &[u8]) -> FieldElement {
//     FieldElement::from_byte_slice_be(bytes)
// }

fn hash(msg: &[u8]) -> AffinePoint {
    let test_swu_to_curve_hasher = MapToCurveBasedHasher::<
        Projective<TestSWUMapToCurveConfig>,
        DefaultFieldHasher<PedersenHash, 128>,
        SWUMap<TestSWUMapToCurveConfig>,
    >::new(&[1])
    .unwrap();

    let p = test_swu_to_curve_hasher.hash(msg).unwrap();
    AffinePoint {
        x: FieldElement::from_mont(Fp::into_bigint(p.x).0),
        y: FieldElement::from_mont(Fp::into_bigint(p.y).0),
        infinity: p.infinity,
    }
}

impl ECVRF {
    pub fn new() -> Self {
        Self {}
    }

    // ECVRF_hash_to_curve_Simplified_SWU
    pub fn hash_to_curve(&self, public_key: &AffinePoint, message: &[u8]) -> AffinePoint {
        let pk_string = EC2OSP(public_key, true);
        let one_string = [0x01];
        hash(
            &[
                &[STARK_SHA256_TAI],
                &one_string,
                pk_string.as_slice(),
                message,
            ]
            .concat(),
        )
    }

    pub fn prove(&self, secret_key: &FieldElement, message: &[u8]) -> Result<(), Error> {
        // Step 1: derive public key from secret key
        let public_key_point = mul_by_bits(&GENERATOR, secret_key);

        // Step 2: Hash to curve
        let h_point = self.hash_to_curve(&public_key_point, message);

        Ok(())

        // // Step 3: point to string
        // let h_string = h_point.to_bytes(
        //     &self.group,
        //     PointConversionForm::COMPRESSED,
        //     &mut self.bn_ctx,
        // )?;

        // // Step 4: Gamma = x * H
        // let mut gamma_point = EcPoint::new(self.group.as_ref())?;
        // gamma_point.mul(self.group.as_ref(), &h_point, &secret_key, &self.bn_ctx)?;

        // // Step 5: nonce
        // let k = self.generate_nonce(&secret_key, &h_string)?;

        // // Step 6: c = hash points(...)
        // let mut u_point = EcPoint::new(self.group.as_ref())?;
        // let mut v_point = EcPoint::new(self.group.as_ref())?;
        // u_point.mul_generator(self.group.as_ref(), &k, &self.bn_ctx)?;
        // v_point.mul(self.group.as_ref(), &h_point, &k, &self.bn_ctx)?;
        // let c = self.hash_points(&[&h_point, &gamma_point, &u_point, &v_point])?;

        // // Step 7: s = (k + c*x) mod q
        // let s = &(&k + &(&c * &secret_key)) % &self.order;

        // // Step 8: encode (gamma, c, s)
        // let gamma_string = gamma_point.to_bytes(
        //     &self.group,
        //     PointConversionForm::COMPRESSED,
        //     &mut self.bn_ctx,
        // )?;
        // // Fixed size; len(c) must be n and len(s)=2n
        // let c_string = append_leading_zeros(&c.to_vec(), self.n);
        // let s_string = append_leading_zeros(&s.to_vec(), self.qlen);
        // // proof =  [Gamma_string||c_string||s_string]
        // let proof = [&gamma_string[..], &c_string, &s_string].concat();

        // Ok(proof)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
    use starknet_crypto::{get_public_key, FieldElement};

    use super::*;

    #[test]
    fn it_works() {
        let mut buf = Vec::new();

        let g = TestSWUMapToCurveConfig::GENERATOR;
        g.serialize_compressed(&mut buf).unwrap();
        println!(
            "generator {g:?} {} {}",
            g.is_on_curve(),
            g.is_in_correct_subgroup_assuming_on_curve()
        );
        println!("buffer {buf:?}");

        let deg = Affine::<TestSWUMapToCurveConfig>::deserialize_compressed(&*buf).unwrap();
        println!("deserialized {deg:?}");
        // let vrf = ECVRF::new();

        // let secret_key = field_element_from_be_hex(
        //     "c9afa9d845ba75166b5c215767b1d6934e50c3db36e89b127b8a622b120f6721",
        // );
        // let public_key = get_public_key(&secret_key);

        // let message: &[u8] = b"sample";

        // let pi = vrf.prove(&secret_key, &message).unwrap();
    }

    fn field_element_from_be_hex(hex: &str) -> FieldElement {
        let decoded = hex::decode(hex.trim_start_matches("0x")).unwrap();

        if decoded.len() > 32 {
            panic!("hex string too long");
        }

        let mut buffer = [0u8; 32];
        buffer[(32 - decoded.len())..].copy_from_slice(&decoded[..]);

        FieldElement::from_bytes_be(&buffer).unwrap()
    }
}
