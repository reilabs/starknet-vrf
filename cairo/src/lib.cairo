use core::ec::{EcPointImpl, stark_curve};
use ecvrf::hash_to_curve;

mod ecvrf;
mod error;
mod math;

fn main() -> (felt252, felt252) {
    let pk = EcPointImpl::new(stark_curve::GEN_X, stark_curve::GEN_Y).unwrap();
    // let gamma = EcPointImpl::new(stark_curve::GEN_X, stark_curve::GEN_Y).unwrap();
    // let c: felt252 = 10;
    // let s: felt252 = 20;

    let mut seed = ArrayTrait::new();
    seed.append(42);
    let (x, y) = hash_to_curve(pk.try_into().unwrap(), seed.span()).unwrap();

    println!("hash_to_curve {x}, {y}");
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn hash_to_curve() {
        main();
    }
}
