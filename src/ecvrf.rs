use ark_ec::{
    hashing::{
        curve_maps::swu::{SWUConfig, SWUMap},
        map_to_curve_hasher::MapToCurve,
    },
    short_weierstrass::{Affine, SWCurveConfig},
    AffineRepr, CurveConfig,
};
use ark_ff::BigInt;
use ark_serialize::CanonicalSerialize;

use crate::error::Error;
use crate::{error::Result, hash::HashToField};

pub const STARK_PEDERSEN_SSWU: u8 = 0xff;

pub type Proof<Curve> = (
    Affine<Curve>,
    <Curve as CurveConfig>::ScalarField,
    <Curve as CurveConfig>::ScalarField,
);

pub struct ECVRF<Curve, Hasher>
where
    Curve: SWCurveConfig + SWUConfig,
    Curve::BaseField: From<BigInt<4>>,
    Curve::ScalarField: From<BigInt<4>>,
    Hasher: HashToField,
{
    suite: u8,
    public_key: Affine<Curve>,
    mapper: SWUMap<Curve>,
    hasher: Hasher,
}

impl<Curve, Hasher> ECVRF<Curve, Hasher>
where
    Curve: SWCurveConfig + SWUConfig,
    Curve::BaseField: From<BigInt<4>>,
    Curve::ScalarField: From<BigInt<4>>,
    Hasher: HashToField,
{
    pub fn new(suite: u8, public_key: Affine<Curve>) -> Result<Self> {
        Ok(Self {
            suite,
            public_key,
            mapper: SWUMap::new()?,
            hasher: Hasher::new(),
        })
    }

    pub fn prove(&self, secret_key: &Curve::ScalarField, alpha: &[u8]) -> Result<Proof<Curve>> {
        let pk_from_secret = Curve::GENERATOR * secret_key;
        if self.public_key != pk_from_secret {
            return Err(Error::InvalidSecretKey);
        }

        let h = self.hash_to_curve(alpha)?;
        let mut h_string = Vec::new();
        h.serialize_compressed(&mut h_string)?;

        let gamma: Affine<Curve> = (h * secret_key).into();
        let k = self.nonce(secret_key, &h_string)?;
        let c = self.hash_points(&[
            self.public_key,
            h,
            gamma,
            (Curve::GENERATOR * k).into(),
            (h * k).into(),
        ])?;
        let s = k + c * secret_key;
        Ok((gamma, c, s))
    }

    pub fn proof_to_hash(&self, proof: &Proof<Curve>) -> Result<Curve::BaseField> {
        let mut string = vec![self.suite, 0x03];

        let mut cofactor_buf: [u64; 4] = [0; 4];
        for (i, limb) in Curve::COFACTOR.iter().enumerate() {
            cofactor_buf[i] = *limb;
        }

        let cofactor_gamma = proof.0.mul_by_cofactor_to_group();
        // our cofactor is 1
        assert_eq!(proof.0, cofactor_gamma);

        let mut buf = Vec::new();
        proof.0.serialize_compressed(&mut buf)?;

        string.extend_from_slice(&buf);
        string.extend_from_slice(&[0x00]);

        Ok(Curve::BaseField::from(self.hasher.hash(&string)))
    }

    pub fn verify(&self, alpha: &[u8], proof: &Proof<Curve>) -> Result<()> {
        let pk = self.public_key;
        let (gamma, c, s) = proof;
        let h = self.hash_to_curve(alpha)?;
        let u = (Curve::GENERATOR * s) - (pk * *c);
        let v = (h * s) - (*gamma * *c);
        let c_prim = self.hash_points(&[pk, h, *gamma, u.into(), v.into()])?;

        if *c == c_prim {
            Ok(())
        } else {
            Err(Error::ProofVerificationError)
        }
    }

    fn hash_to_curve(&self, message: &[u8]) -> Result<Affine<Curve>> {
        let pk = self.public_key;
        let mut pk_string = Vec::new();
        pk.serialize_compressed(&mut pk_string)?;
        let t_string = [&[self.suite, 0x01], pk_string.as_slice(), message].concat();
        let t = self.hasher.hash(&t_string);
        Ok(self.mapper.map_to_curve(Curve::BaseField::from(t))?)
    }

    fn hash_points(&self, points: &[Affine<Curve>]) -> Result<Curve::ScalarField> {
        let mut string = vec![self.suite, 0x02];
        for point in points {
            let mut buf = Vec::new();
            point.serialize_compressed(&mut buf)?;
            string.extend_from_slice(&buf);
        }
        string.extend_from_slice(&[0x00]);

        // TODO: typically challenges have half the number of bits of the
        // scalar field.
        // for now this returns a full scalar field value
        let fr = self.hasher.hash(&string);
        Ok(Curve::ScalarField::from(fr))
    }

    // 5.4.2.2. ECVRF Nonce Generation from RFC 8032
    pub fn nonce(
        &self,
        secret_key: &Curve::ScalarField,
        message: &[u8],
    ) -> Result<Curve::ScalarField> {
        let mut sk_string = Vec::new();
        secret_key.serialize_compressed(&mut sk_string)?;

        let fr = self.hasher.hash(&[sk_string.as_slice(), message].concat());
        Ok(Curve::ScalarField::from(fr))
    }
}
