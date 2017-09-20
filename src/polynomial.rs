use std::ops::{Add, Sub, Mul, Div, Rem};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use std::fmt;

/// Struct representing a binary polynomial (that is, one with coefficients
/// that are either 0 or 1). Note that the coefficients are elements of the
/// Z_2 field, so 1+1 = 0.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BinaryPolynomial(pub u64);

impl BinaryPolynomial {
    pub fn degree(&self) -> i8 {
        let mut result = -1;
        let mut x = self.0;
        while x > 0 {
            result += 1;
            x >>= 1;
        }
        result
    }
}

impl Add<BinaryPolynomial> for BinaryPolynomial {
    type Output = BinaryPolynomial;

    fn add(self, other: BinaryPolynomial) -> BinaryPolynomial {
        BinaryPolynomial(self.0 ^ other.0)
    }
}

impl AddAssign<BinaryPolynomial> for BinaryPolynomial {
    fn add_assign(&mut self, other: BinaryPolynomial) {
        *self = *self + other;
    }
}

impl Sub<BinaryPolynomial> for BinaryPolynomial {
    type Output = BinaryPolynomial;

    fn sub(self, other: BinaryPolynomial) -> BinaryPolynomial {
        BinaryPolynomial(self.0 ^ other.0)
    }
}

impl SubAssign<BinaryPolynomial> for BinaryPolynomial {
    fn sub_assign(&mut self, other: BinaryPolynomial) {
        *self = *self - other;
    }
}

impl Mul<BinaryPolynomial> for BinaryPolynomial {
    type Output = BinaryPolynomial;

    fn mul(self, other: BinaryPolynomial) -> BinaryPolynomial {
        let mut x = other.0;
        let mut y = self.0;
        let mut result = 0;
        while x > 0 && y > 0 {
            if x & 1 == 1 {
                result ^= y;
            }
            y <<= 1;
            x >>= 1;
        }
        BinaryPolynomial(result)
    }
}

impl MulAssign<BinaryPolynomial> for BinaryPolynomial {
    fn mul_assign(&mut self, other: BinaryPolynomial) {
        *self = *self * other;
    }
}

impl Div<BinaryPolynomial> for BinaryPolynomial {
    type Output = BinaryPolynomial;

    fn div(self, other: BinaryPolynomial) -> BinaryPolynomial {
        if other == BinaryPolynomial(0) {
            panic!("Division by zero!");
        }
        let mut result = BinaryPolynomial(0);
        let mut x = self;
        while x.degree() >= other.degree() {
            let q = BinaryPolynomial(1 << (x.degree() - other.degree()));
            result += q;
            x -= other * q;
        }
        result
    }
}

impl DivAssign<BinaryPolynomial> for BinaryPolynomial {
    fn div_assign(&mut self, other: BinaryPolynomial) {
        *self = *self / other;
    }
}

impl Rem<BinaryPolynomial> for BinaryPolynomial {
    type Output = BinaryPolynomial;

    fn rem(self, other: BinaryPolynomial) -> BinaryPolynomial {
        if other == BinaryPolynomial(0) {
            panic!("Division by zero!");
        }
        let mut x = self;
        while x.degree() >= other.degree() {
            let q = BinaryPolynomial(1 << (x.degree() - other.degree()));
            x -= other * q;
        }
        x
    }
}

impl RemAssign<BinaryPolynomial> for BinaryPolynomial {
    fn rem_assign(&mut self, other: BinaryPolynomial) {
        *self = *self % other;
    }
}

impl fmt::Debug for BinaryPolynomial {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut components = Vec::new();
        let mut degree = 63u8;
        let mut x = self.0;
        while x > 0 {
            if x & (1u64 << 63) > 0 {
                if degree > 1 {
                    components.push(format!("x^{}", degree));
                } else if degree == 1 {
                    components.push("x".to_string());
                } else {
                    components.push("1".to_string());
                }
            }
            if degree > 0 {
                degree -= 1;
            }
            x <<= 1;
        }
        let result = if components.is_empty() {
            "0".to_string()
        } else {
            components.join(" + ")
        };
        write!(formatter, "{}", result)
    }
}

#[cfg(test)]
mod test {
    use super::BinaryPolynomial;

    #[test]
    fn test_add() {
        let _1 = BinaryPolynomial(0b1);
        let _x = BinaryPolynomial(0b10);
        assert_eq!(_1 + _1, BinaryPolynomial(0b0));
        assert_eq!(_1 + _x, BinaryPolynomial(0b11));
        assert_eq!(_x + _x, BinaryPolynomial(0b0));
    }

    #[test]
    fn test_mul() {
        let _1 = BinaryPolynomial(0b1);
        let _x = BinaryPolynomial(0b10);
        let _x2 = BinaryPolynomial(0b100);
        assert_eq!(_1 * _x, _x);
        assert_eq!(_x * _x, _x2);
        assert_eq!((_x + _1) * _x, _x2 + _x);
    }

    #[test]
    fn test_div_rem() {
        let _x2_x_1 = BinaryPolynomial(0b111);
        let _x4 = BinaryPolynomial(0b10000);
        assert_eq!(_x4 / _x2_x_1, BinaryPolynomial(0b110));
        assert_eq!(_x4 % _x2_x_1, BinaryPolynomial(0b10));
        assert_eq!(_x2_x_1 / _x4, BinaryPolynomial(0b0));
        assert_eq!(_x2_x_1 % _x4, _x2_x_1);
    }

    #[test]
    fn test_debug() {
        let _1 = BinaryPolynomial(0b1);
        let _x = BinaryPolynomial(0b10);
        let _x2 = BinaryPolynomial(0b100);
        assert_eq!(format!("{:?}", _1), "1");
        assert_eq!(format!("{:?}", _x), "x");
        assert_eq!(format!("{:?}", _x2), "x^2");
        assert_eq!(format!("{:?}", _x + _1), "x + 1");
        assert_eq!(format!("{:?}", _x2 + _1), "x^2 + 1");
        assert_eq!(format!("{:?}", _x2 * _x), "x^3");
        assert_eq!(format!("{:?}", BinaryPolynomial(0)), "0");
    }
}
