use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ec::CurveGroup;
use cairo_lang_runner::StarknetHintProcessor;
use stark_vrf::{base_field_from_field_element, scalar_field_from_field_element, field_element_from_base_field, field_element_from_scalar_field, ScalarField, StarkCurve, StarkVRF};
use starknet_ff::FieldElement;

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

impl Default for StarknetHintVrf {
    fn default() -> Self {
        let secret = FieldElement::from_dec_str("42").unwrap();
        Self::new(secret)
    }
}

impl StarknetHintProcessor for StarknetHintVrf {
    fn matches_selector(&self, selector: &str) -> bool {
        selector == "vrf"
    }

    fn execute(
        &self,
        _selector: &str,
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
}
