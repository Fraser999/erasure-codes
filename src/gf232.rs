use polynomial::BinaryPolynomial;
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

/// This is an irreducible degree-32 polynomial: X^32 + X^7 + X^3 + X^2 + 1
const IRR232: BinaryPolynomial = BinaryPolynomial(0x10000008D);

/// A struct representing an element of the `GF(2^32)` Galois field.
/// The elements of this field are binary polynomials mod an irreducible
/// polynomial: IRR232
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GF232(pub u32);

impl GF232 {
    /// Finds an inverse of the element using Blankinship's algorithm
    pub fn inverse(&self) -> GF232 {
        if *self == GF232(0) {
            panic!("Division by zero!");
        }
        let mut c = IRR232;
        let mut d = BinaryPolynomial(self.0 as u64);
        let mut n1 = BinaryPolynomial(0);
        let mut n = BinaryPolynomial(1);
        loop {
            let q = c / d;
            let r = c % d;
            if r == BinaryPolynomial(0) {
                break;
            }
            c = d;
            d = r;
            let t = n1;
            n1 = n;
            n = t - q * n;
        }
        GF232(n.0 as u32)
    }
}

impl Add<GF232> for GF232 {
    type Output = GF232;

    fn add(self, other: GF232) -> GF232 {
        GF232(self.0 ^ other.0)
    }
}

impl AddAssign<GF232> for GF232 {
    fn add_assign(&mut self, other: GF232) {
        *self = *self + other;
    }
}

impl Sub<GF232> for GF232 {
    type Output = GF232;

    fn sub(self, other: GF232) -> GF232 {
        GF232(self.0 ^ other.0)
    }
}

impl SubAssign<GF232> for GF232 {
    fn sub_assign(&mut self, other: GF232) {
        *self = *self - other;
    }
}

impl Mul<GF232> for GF232 {
    type Output = GF232;

    fn mul(self, other: GF232) -> GF232 {
        let poly1 = BinaryPolynomial(self.0 as u64);
        let poly2 = BinaryPolynomial(other.0 as u64);
        let res = (poly1 * poly2) % IRR232;
        GF232(res.0 as u32)
    }
}

impl MulAssign<GF232> for GF232 {
    fn mul_assign(&mut self, other: GF232) {
        *self = *self * other;
    }
}

impl Div<GF232> for GF232 {
    type Output = GF232;

    fn div(self, other: GF232) -> GF232 {
        self * other.inverse()
    }
}

impl DivAssign<GF232> for GF232 {
    fn div_assign(&mut self, other: GF232) {
        *self = *self / other;
    }
}

#[cfg(test)]
mod test {
    use super::GF232;

    #[test]
    fn test_inverse() {
        for i in 1..10000 {
            let x = GF232(i);
            assert_eq!(
                x * x.inverse(),
                GF232(1),
                "x = {:?}, x-1 = {:?}",
                x,
                x.inverse()
            );
        }
    }
}
