# Long Arithmetic

This crate exports `BigUInt` and `BigInt` types with are represented as `Vec<u64>`. `BigInt` is a wrapper around `BigUInt` with `sign` field added to it. These types are made for arbitrary-precision calculations.

Common operations like `Add`, `Sub`, `Mul`, `Div`, shifts are overloaded. `pow` method of trait `Pow` represents raising `self` to some power. Types also can be parsed from strings and can be displayed. Ordering traits are implemented for both `BigUInt` and `BigInt`

All tests can be run using `cargo test`

`Mul` trait uses `O(n^2)` time implementation. Also multiplication by Karatsuba algorithm is implemented in `karatruba` module. It has `O(n^log(2,3))` time complexity. FFT has not yet been implemented.
