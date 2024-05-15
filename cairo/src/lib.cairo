use core::num::traits::zero::Zero;
use core::array::ArrayTrait;
use core::poseidon::poseidon_hash_span;
use core::ec::{EcPointImpl, NonZeroEcPoint, EcPointTryIntoNonZero, EcPoint, stark_curve, ec_point_unwrap};
use core::bytes_31::Bytes31IntoFelt252;

pub extern fn felt252_div(lhs: felt252, rhs: NonZero<felt252>) -> felt252 nopanic;

#[derive(Drop)]
enum Error {}

// Sage script to find Z
//
// # Arguments:
// # - F, a field object, e.g., F = GF(2^521 - 1)
// # - A and B, the coefficients of the curve y^2 = x^3 + A * x + B
// def find_z_sswu(F, A, B):
//     R.<xx> = F[]                       # Polynomial ring over F
//     g = xx^3 + F(A) * xx + F(B)        # y^2 = g(x) = x^3 + A * x + B
//     ctr = F.gen()
//     while True:
//         for Z_cand in (F(ctr), F(-ctr)):
//             # Criterion 1: Z is non-square in F.
//             if is_square(Z_cand):
//                 continue
//             # Criterion 2: Z != -1 in F.
//             if Z_cand == F(-1):
//                 continue
//             # Criterion 3: g(x) - Z is irreducible over F.
//             if not (g - Z_cand).is_irreducible():
//                 continue
//             # Criterion 4: g(B / (Z * A)) is square in F.
//             if is_square(g(B / (Z_cand * A))):
//                 return Z_cand
//         ctr += 1

// find_z_sswu(FiniteField(P), stark_curve::ALPHA, stark_curve::BETA)
pub const Z: felt252 = 19;

// constants
pub const A: felt252 = stark_curve::ALPHA;
pub const B: felt252 = stark_curve::BETA;

// constants for sqrt_ration
pub const c1: felt252 = 192;                // largest integer such that 2^c1 divides q - 1;
pub const c2: felt252 = 576460752303423505; // (q - 1) / 2^c1
pub const c3: u256 = 288230376151711752; // (c2 - 1) / 2
pub const c4: u256 = 6277101735386680763835789423207666416102355444464034512895; // 2^c1 - 1
pub const c5: u256 = 3138550867693340381917894711603833208051177722232017256448; // 2^(c1 - 1)
pub const c6: felt252 = 271122989245172633851355451947103605954274810690058238200515407597964208139; // Z^c2;
pub const c7: felt252 = 3564856802633786767843881147069258360232483436295672590729615622861023547560; // Z^((c2 + 1) / 2)

fn main() -> (felt252, felt252) {
    // let generator = EcPointImpl::new(stark_curve::GEN_X, stark_curve::GEN_Y).unwrap();
    let pk = EcPointImpl::new(stark_curve::GEN_X, stark_curve::GEN_Y).unwrap();
    // let gamma = EcPointImpl::new(stark_curve::GEN_X, stark_curve::GEN_Y).unwrap();
    // let c: felt252 = 10;
    // let s: felt252 = 20;

    let mut seed = ArrayTrait::new();
    seed.append(42);
    let (x, y) = hash_to_curve(pk.try_into().unwrap(), seed.span()).unwrap();

    // println!("hash_to_curve {x}, {y}");
    (x, y)
}

fn hash_to_curve(pk: NonZeroEcPoint, a: Span<felt252>) -> Result<(felt252, felt252), Error> {
    let (x, y) = ec_point_unwrap(pk);

    let mut buf = ArrayTrait::new();
    buf.append(x);
    buf.append(y);
    buf.append(1);
    buf.append_span(a);

    let mut hash = poseidon_hash_span(buf.span());
    // println!("buf: {buf:?} hash: {hash}");
    
    map_to_curve(hash)
}

pub fn fast_power(
    base: felt252, mut power: u256
) -> felt252 {
    assert!(base != 0_u8.into(), "fast_power: invalid input");

    let mut base: felt252 = base.into();
    let mut result: felt252 = 1;

    loop {
        if power % 2_u8.into() != 0_u8.into() {
            result *= base;
        }
        power /= 2_u8.into();
        if (power == 0_u8.into()) {
            break;
        }
        base *= base;
    };

    result.try_into().expect('too large to fit output type')
}

// Input: u and v, elements of F, where v != 0.
// Output: (b, y), where
//   b = True and y = sqrt(u / v) if (u / v) is square in F, and
//   b = False and y = sqrt(Z * (u / v)) otherwise.
fn sqrt_ratio(u: felt252, v: felt252) -> (bool, felt252) {
    // 1. tv1 = c6
    let mut tv1 = c6;
    // 2. tv2 = v^c4
    let tv2 = fast_power(v, c4);
    // 3. tv3 = tv2^2
    let tv3 = tv2 * tv2;
    // 4. tv3 = tv3 * v
    let tv3 = tv3 * v;
    // 5. tv5 = u * tv3
    let tv5 = u * tv3;
    // 6. tv5 = tv5^c3
    let tv5 = fast_power(tv5, c3);
    // 7. tv5 = tv5 * tv2
    let tv5 = tv5 * tv2;
    // 8. tv2 = tv5 * v
    let tv2 = tv5 * v;
    // 9. tv3 = tv5 * u
    let mut tv3 = tv5 * u;
    // 10. tv4 = tv3 * tv2
    let mut tv4 = tv3 * tv2;
    // 11. tv5 = tv4^c5
    let tv5 = fast_power(tv4, c5);
    // 12. isQR = tv5 == 1
    let isQR = tv5 == 1;
    let mut e1 = tv5 == 1;
    // 13. tv2 = tv3 * c7
    let mut tv2 = tv3 * c7;
    // 14. tv5 = tv4 * tv1
    let mut tv5 = tv4 * tv1;
    // 15. tv3 = CMOV(tv2, tv3, isQR)
    // 16. tv4 = CMOV(tv5, tv4, isQR)
    if !isQR {
        tv3 = tv2;
        tv4 = tv5;
    }
    // 17. for i in (c1, c1 - 1, ..., 2):
    let mut i = c1;
    loop {
        if i == 1 { break; }

        // 18.    tv5 = i - 2
        tv5 = i - 2;
        // 19.    tv5 = 2^tv5
        tv5 = fast_power(2, tv5.into());
        // 20.    tv5 = tv4^tv5
        tv5 = fast_power(tv4, tv5.into());
        // 21.    e1 = tv5 == 1
        e1 = tv5 == 1;
        // 22.    tv2 = tv3 * tv1
        tv2 = tv3 * tv1;
        // 23.    tv1 = tv1 * tv1
        tv1 = tv1 * tv1;
        // 24.    tv5 = tv4 * tv1
        tv5 = tv4 * tv1;
        // 25.    tv3 = CMOV(tv2, tv3, e1)
        // 26.    tv4 = CMOV(tv5, tv4, e1)    
        if !e1 {
            tv3 = tv2;
            tv4 = tv5;
        }

        i -= 1;
    };
    // 27. return (isQR, tv3)
    (isQR, tv3)
}

fn map_to_curve(u: felt252) -> Result<(felt252, felt252), Error> {
    // map_to_curve_simple_swu(u)

    // Input: u, an element of F.
    // Output: (x, y), a point on E.

    // Steps:
    // 1.  tv1 = u^2
    // 2.  tv1 = Z * tv1
    let tv1 = Z * u * u;

    // 3.  tv2 = tv1^2
    // 4.  tv2 = tv2 + tv1
    let tv2 = tv1 * tv1 + tv1;

    // 5.  tv3 = tv2 + 1
    // 6.  tv3 = B * tv3
    let tv3 = B * (tv2 + 1);

    // CMOV(false_value, true_value, condition)
    // 7.  tv4 = CMOV(Z, -tv2, tv2 != 0)
    let tv4 = if tv2.is_zero() {
        Z
    } else {
        -tv2
    };

    // 8.  tv4 = A * tv4
    let tv4 = A * tv4;

    // 9.  tv2 = tv3^2
    let tv2 = tv3 * tv3;

    // 10. tv6 = tv4^2
    let tv6 = tv4 * tv4;

    // 11. tv5 = A * tv6
    let tv5 = A * tv6;

    // 12. tv2 = tv2 + tv5
    let tv2 = tv2 + tv5;

    // 13. tv2 = tv2 * tv3
    let tv2 = tv2 * tv3;

    // 14. tv6 = tv6 * tv4
    let tv6 = tv6 * tv4;

    // 15. tv5 = B * tv6
    let tv5 = B * tv6;

    // 16. tv2 = tv2 + tv5
    let tv2 = tv2 + tv5;

    // 17.   x = tv1 * tv3
    let x = tv1 * tv3;

    // 18. (is_gx1_square, y1) = sqrt_ratio(tv2, tv6)
    let (is_gx1_square, y1) = sqrt_ratio(tv2, tv6);

    // 19.   y = tv1 * u
    let y = tv1 * u;

    // 20.   y = y * y1
    let y = y * y1;

    // 21.   x = CMOV(x, tv3, is_gx1_square)
    // 22.   y = CMOV(y, y1, is_gx1_square)
    let (x, y) = if is_gx1_square {
        (tv3, y1)
    } else {
        (x, y)
    };

    // 23.  e1 = sgn0(u) == sgn0(y)
    // 24.   y = CMOV(-y, y, e1)
    let u_256: u256 = u.into();
    let y_256: u256 = y.into();
    let y = if (u_256 % 2) == (y_256 % 2) {
        y
    } else {
        -y
    };

    // 25.   x = x / tv4
    let x = felt252_div(x, tv4.try_into().unwrap());

    // 26. return (x, y)
    Result::Ok((x, y))
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn hash_to_curve() {
        main();
    }
}
