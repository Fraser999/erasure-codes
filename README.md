# Erasure codes

This repository contains a small proof-of-concept program testing erasure coding using polynomials on
a Galois field `GF(2^32)`.

Contents:

* `src/polynomial.rs` - a minimal implementation of polynomials on `Z_2` encoded as 64-bit unsigned numbers. `i`-th bit represents the coefficient multiplying `x^i`.
* `src/gf232.rs` - an implementation of the Galois field `GF(2^32)` using binary polynomials.
* `src/polygf232.rs` - an implementation of polynomials on the Galois field, used for interpolation in the codec
* `src/codec.rs` - the actual implementation of the encoding and decoding routines, along with some tests
* `src/main.rs` - a program generating a degree-32 irreducible polynomial, necessary for the implementation of the Galois field
