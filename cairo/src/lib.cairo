mod ecvrf;
mod error;
mod math;

fn main() -> felt252 {
    42
}

#[cfg(test)]
mod tests {
    use core::option::OptionTrait;
use core::ec::{EcPointImpl, stark_curve};
    use super::ecvrf::{hash_to_curve, Proof, ECVRFImpl};

    fn proof_from_oracle() -> Proof {
        Proof {
            gamma: EcPointImpl::new(
                1506339363762384048749124975867331702319430609263271304275332020910807468800,
                36259598506905210600179635686591002688831785399437338349196739602416217657
            ).unwrap(),
            c: 2613903846701008054856365693011070443633034612733309583190565217827378733393,
            s: 1867682679224997956048283066055885717352683300581532690215097247223135564277,
        }
    }
    
    #[test]
    fn ecvrf_verify() {
        let pk = EcPointImpl::new(
            2465182048640915825114623967805639036884813714770257338089158027381626459289,
            3038635738014387716559859267483610492356329532552881764846792983975787300333
        ).unwrap(); 
        let proof = proof_from_oracle();
        let ecvrf = ECVRFImpl::new(pk);
        let mut seed = ArrayTrait::new();
        seed.append(42);
    
        ecvrf.verify(proof, seed.span()).unwrap();
    }
}
