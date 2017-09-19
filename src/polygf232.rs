use gf232::GF232;
use std::cmp::max;
use std::ops::{Add, Sub, Mul, Div};
use std::iter::IntoIterator;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PolyGF232(Vec<GF232>);

impl PolyGF232 {
    pub fn new<I: IntoIterator<Item = GF232>>(x: I) -> PolyGF232 {
        let mut result = PolyGF232(x.into_iter().collect());
        result.normalize();
        result
    }

    pub fn apply(&self, x: GF232) -> GF232 {
        let mut powx = GF232(1);
        let mut result = GF232(0);
        for a in &self.0 {
            result += *a * powx;
            powx *= x;
        }
        result
    }

    fn degree(&self) -> isize {
        let mut deg = self.0.len() as isize - 1;
        while deg >= 0 && self.0[deg as usize] == GF232(0) {
            deg -= 1;
        }
        deg
    }

    fn get_coeff(&self, idx: usize) -> GF232 {
        if idx >= self.0.len() {
            GF232(0)
        } else {
            self.0[idx]
        }
    }

    fn get_coeff_mut(&mut self, idx: usize) -> &mut GF232 {
        if idx >= self.0.len() {
            for _ in 0..(idx - self.0.len() + 1) {
                self.0.push(GF232(0));
            }
        }
        &mut self.0[idx]
    }

    fn normalize(&mut self) {
        let deg = self.degree();
        self.0.truncate((deg + 1) as usize);
    }
}

impl Add<GF232> for PolyGF232 {
    type Output = PolyGF232;

    fn add(mut self, other: GF232) -> PolyGF232 {
        let poly = PolyGF232(vec![other]);
        self + poly
    }
}

impl<'a> Add<GF232> for &'a PolyGF232 {
    type Output = PolyGF232;

    fn add(self, other: GF232) -> PolyGF232 {
        let poly = PolyGF232(vec![other]);
        self.clone() + poly
    }
}

impl<'a> Add<&'a PolyGF232> for PolyGF232 {
    type Output = PolyGF232;

    fn add(mut self, other: &'a PolyGF232) -> PolyGF232 {
        for i in 0..max(self.degree(), other.degree()) + 1 {
            *self.get_coeff_mut(i as usize) = self.get_coeff(i as usize) +
                other.get_coeff(i as usize);
        }
        self.normalize();
        self
    }
}

impl Add<PolyGF232> for PolyGF232 {
    type Output = PolyGF232;

    fn add(self, other: PolyGF232) -> PolyGF232 {
        self.add(&other)
    }
}

impl<'a> Add<PolyGF232> for &'a PolyGF232 {
    type Output = PolyGF232;

    fn add(self, other: PolyGF232) -> PolyGF232 {
        self.clone().add(other)
    }
}

impl<'a, 'b> Add<&'b PolyGF232> for &'a PolyGF232 {
    type Output = PolyGF232;

    fn add(self, other: &'b PolyGF232) -> PolyGF232 {
        self.clone().add(other)
    }
}

impl<'a> Sub<&'a PolyGF232> for PolyGF232 {
    type Output = PolyGF232;

    fn sub(mut self, other: &'a PolyGF232) -> PolyGF232 {
        for i in 0..max(self.degree(), other.degree()) + 1 {
            *self.get_coeff_mut(i as usize) = self.get_coeff(i as usize) -
                other.get_coeff(i as usize);
        }
        self.normalize();
        self
    }
}

impl Sub<PolyGF232> for PolyGF232 {
    type Output = PolyGF232;

    fn sub(self, other: PolyGF232) -> PolyGF232 {
        self.sub(&other)
    }
}

impl<'a> Sub<PolyGF232> for &'a PolyGF232 {
    type Output = PolyGF232;

    fn sub(self, other: PolyGF232) -> PolyGF232 {
        self.clone().sub(other)
    }
}

impl<'a, 'b> Sub<&'b PolyGF232> for &'a PolyGF232 {
    type Output = PolyGF232;

    fn sub(self, other: &'b PolyGF232) -> PolyGF232 {
        self.clone().sub(other)
    }
}

impl Mul<GF232> for PolyGF232 {
    type Output = PolyGF232;

    fn mul(mut self, other: GF232) -> PolyGF232 {
        for x in self.0.iter_mut() {
            *x *= other;
        }
        self.normalize();
        self
    }
}

impl<'a> Mul<GF232> for &'a PolyGF232 {
    type Output = PolyGF232;

    fn mul(self, other: GF232) -> PolyGF232 {
        self.clone() * other
    }
}

impl<'a> Mul<&'a PolyGF232> for PolyGF232 {
    type Output = PolyGF232;

    fn mul(self, other: &'a PolyGF232) -> PolyGF232 {
        let mut result = PolyGF232(vec![]);
        let mut start = vec![];
        for x in self.0 {
            let mut comp = start.clone();
            for y in &other.0 {
                comp.push(*y * x);
            }
            result = result + PolyGF232(comp);
            start.push(GF232(0));
        }
        result
    }
}

impl Mul<PolyGF232> for PolyGF232 {
    type Output = PolyGF232;

    fn mul(self, other: PolyGF232) -> PolyGF232 {
        self.mul(&other)
    }
}

impl<'a> Mul<PolyGF232> for &'a PolyGF232 {
    type Output = PolyGF232;

    fn mul(self, other: PolyGF232) -> PolyGF232 {
        self.clone().mul(other)
    }
}

impl<'a, 'b> Mul<&'b PolyGF232> for &'a PolyGF232 {
    type Output = PolyGF232;

    fn mul(self, other: &'b PolyGF232) -> PolyGF232 {
        self.clone().mul(other)
    }
}

impl Div<GF232> for PolyGF232 {
    type Output = PolyGF232;

    fn div(mut self, other: GF232) -> PolyGF232 {
        for x in self.0.iter_mut() {
            *x /= other;
        }
        self.normalize();
        self
    }
}

impl<'a> Div<GF232> for &'a PolyGF232 {
    type Output = PolyGF232;

    fn div(self, other: GF232) -> PolyGF232 {
        self.clone() / other
    }
}

impl<'a> Div<&'a PolyGF232> for PolyGF232 {
    type Output = PolyGF232;

    fn div(mut self, other: &'a PolyGF232) -> PolyGF232 {
        if *other == PolyGF232(vec![]) {
            panic!("Division by 0!");
        }
        let mut result = vec![];
        while self.degree() > other.degree() && self.degree() > -1 {
            let q = self.0[self.degree() as usize] / other.0[other.degree() as usize];
            result.push(q);
            self = self - other * q;
        }
        result.reverse();
        PolyGF232(result)
    }
}

impl Div<PolyGF232> for PolyGF232 {
    type Output = PolyGF232;

    fn div(self, other: PolyGF232) -> PolyGF232 {
        self.div(&other)
    }
}

impl<'a> Div<PolyGF232> for &'a PolyGF232 {
    type Output = PolyGF232;

    fn div(self, other: PolyGF232) -> PolyGF232 {
        self.clone().div(other)
    }
}

impl<'a, 'b> Div<&'b PolyGF232> for &'a PolyGF232 {
    type Output = PolyGF232;

    fn div(self, other: &'b PolyGF232) -> PolyGF232 {
        self.clone().div(other)
    }
}
