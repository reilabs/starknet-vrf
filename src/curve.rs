use ark_ec::{
    hashing::curve_maps::swu::SWUConfig,
    short_weierstrass::{Affine, SWCurveConfig},
    CurveConfig,
};
use ark_ff::{Fp256, MontBackend, MontConfig, MontFp};

#[derive(MontConfig)]
#[modulus = "3618502788666131213697322783095070105623107215331596699973092056135872020481"]
#[generator = "3"]
pub struct FrConfig;

#[derive(MontConfig)]
#[modulus = "3618502788666131213697322783095070105526743751716087489154079457884512865583"]
#[generator = "3"]
pub struct ScalarFieldConfig;

pub type Fr = Fp256<MontBackend<FrConfig, 4>>;
pub type ScalarField = Fp256<MontBackend<ScalarFieldConfig, 4>>;

pub struct StarkCurve;

impl CurveConfig for StarkCurve {
    const COFACTOR: &'static [u64] = &[1];
    const COFACTOR_INV: ScalarField = MontFp!("1");

    type BaseField = Fr;
    type ScalarField = ScalarField;
}

impl SWCurveConfig for StarkCurve {
    /// COEFF_A = 1
    const COEFF_A: Fr = MontFp!("1");

    /// COEFF_B = 63
    const COEFF_B: Fr =
        MontFp!("3141592653589793238462643383279502884197169399375105820974944592307816406665");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const GENERATOR: Affine<Self> = Affine {
        x: MontFp!("874739451078007766457464989774322083649278607533249481151382481072868806602"),
        y: MontFp!("152666792071518830868575557812948353041420400780739481342941381225525861407"),
        infinity: false,
    };
}

impl SWUConfig for StarkCurve {
    const ZETA: Fr = MontFp!("3");
}
