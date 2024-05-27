use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ec::CurveGroup;
use ark_ff::fields::Field;
use cairo_lang_runner::StarknetHintProcessor;
use stark_vrf::{base_field_from_field_element, scalar_field_from_field_element, field_element_from_base_field, field_element_from_scalar_field, ScalarField, StarkCurve, StarkVRF};
use starknet_ff::FieldElement;
use num_traits::identities::Zero;

pub struct StarknetHintVrf {
    secret: ScalarField,
    vrf: StarkVRF,
}

impl StarknetHintVrf {
    pub fn new(secret: FieldElement) -> Self {
        let secret = scalar_field_from_field_element(&secret);
        let g = StarkCurve::GENERATOR;
        let pk = g * secret;
        Self {
            secret,
            vrf: StarkVRF::new(pk.into_affine()).unwrap()
        }
    }
}

impl StarknetHintVrf {
    fn execute_vrf(
        &self,
        input: &[FieldElement]
    ) -> Vec<FieldElement> {
        let seed: Vec<_> = input.iter().map(|x| base_field_from_field_element(x)).collect();
        let proof = self.vrf.prove(&self.secret, &seed).unwrap();
        
        let pk = StarkCurve::GENERATOR * self.secret;
        println!("executing StarknetHintVrf with pk = {pk} and seed {seed:?}");
        println!("executing StarknetHintVrf result = {proof:?}");

        self.vrf.verify(&proof, &seed).unwrap();

        vec![
            field_element_from_base_field(&proof.0.x),
            field_element_from_base_field(&proof.0.y),
            field_element_from_scalar_field(&proof.1),
            field_element_from_scalar_field(&proof.2),
        ]
    }

    fn execute_sqrt_ratio(
        &self,
        input: &[FieldElement]
    ) -> Vec<FieldElement> {
        let input: Vec<_> = input.iter().map(|x| base_field_from_field_element(x)).collect();
        println!("ruist hint inputs collected {input:?}");

        let u = input[0];
        let v = input[1];
        let z = input[2];
        println!("ruist hint inputs extracted");

        assert!(!v.is_zero());

        println!("ruist hint non zero divisor");

        let gx1 = u / v;
        let result = if gx1.legendre().is_qr() {
            println!("ruist hint quadratic residue");

            gx1.sqrt()
                .expect("We have checked that gx1 is a quadratic residue. Q.E.D")
        } else {
            println!("ruist hint non-quadratic residue");

            let zeta_gx1 = z * gx1;
            zeta_gx1.sqrt().expect(
                "ZETA * gx1 is a quadratic residue because legard is multiplicative. Q.E.D",
            )
        };

        println!("ruist hint finishing");
        vec![field_element_from_base_field(&result)]
    }
}

impl Default for StarknetHintVrf {
    fn default() -> Self {
        let secret = FieldElement::from_dec_str("42").unwrap();
        Self::new(secret)
    }
}

impl StarknetHintProcessor for StarknetHintVrf {
    fn matches_selector(&self, selector: &str) -> bool {
        selector == "vrf" || selector == "sqrt_ratio"
    }

    fn execute(
        &self,
        selector: &str,
        input: &[FieldElement]
    ) -> Vec<FieldElement> {
        if selector == "vrf" {
            self.execute_vrf(input)
        } else if selector == "sqrt_ratio" {
            self.execute_sqrt_ratio(input)
        } else {
            panic!("unknown selector {selector}");
        }
    }

}
