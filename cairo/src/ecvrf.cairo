
use core::ec::{NonZeroEcPoint, EcPointTryIntoNonZero, EcPoint, stark_curve, ec_point_unwrap};
use core::num::traits::zero::Zero;
use core::poseidon::poseidon_hash_span;
use super::math::{Z, A, B, sqrt_ratio};
use super::error::Error;

pub extern fn felt252_div(lhs: felt252, rhs: NonZero<felt252>) -> felt252 nopanic;

pub fn hash_to_curve(pk: NonZeroEcPoint, a: Span<felt252>) -> Result<(felt252, felt252), Error> {
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

// map_to_curve_simple_swu(u)
//   Input: u, an element of F.
//   Output: (x, y), a point on E.
fn map_to_curve(u: felt252) -> Result<(felt252, felt252), Error> {
    let tv1 = Z * u * u;
    let tv2 = tv1 * tv1 + tv1;
    let tv3 = B * (tv2 + 1);
    let tv4 = if tv2.is_zero() {
        Z
    } else {
        -tv2
    };
    let tv4 = A * tv4;
    let tv2 = tv3 * tv3;
    let tv6 = tv4 * tv4;
    let tv5 = A * tv6;
    let tv2 = tv2 + tv5;
    let tv2 = tv2 * tv3;
    let tv6 = tv6 * tv4;
    let tv5 = B * tv6;
    let tv2 = tv2 + tv5;
    let x = tv1 * tv3;
    let (is_gx1_square, y1) = sqrt_ratio(tv2, tv6);
    let y = tv1 * u;
    let y = y * y1;
    let (x, y) = if is_gx1_square {
        (tv3, y1)
    } else {
        (x, y)
    };

    let u_256: u256 = u.into();
    let y_256: u256 = y.into();
    let y = if (u_256 % 2) == (y_256 % 2) {
        y
    } else {
        -y
    };

    let x = felt252_div(x, tv4.try_into().unwrap());
    Result::Ok((x, y))
}
