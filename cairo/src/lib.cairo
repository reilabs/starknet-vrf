mod ecvrf;
mod error;
mod math;

fn main() -> felt252 {
    42
}

#[cfg(test)]
mod tests {
    use core::ec::{EcPointImpl, stark_curve};
    use super::ecvrf::{hash_to_curve, Proof, ECVRFImpl};

    fn fake_proof() -> Proof {
        Proof {
            gamma: EcPointImpl::new(stark_curve::GEN_X, stark_curve::GEN_Y).unwrap(),
            c: 10,
            s: 20,
        }
    }
    
    #[test]
    fn ecvrf_verify() {
        let proof = fake_proof();
        let ecvrf = ECVRFImpl::new();
        let mut seed = ArrayTrait::new();
        seed.append(42);
    
        ecvrf.verify(proof, seed.span()).unwrap();
    }
}
