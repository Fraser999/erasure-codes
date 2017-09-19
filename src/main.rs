extern crate rand;

mod polynomial;
mod gf232;
mod polygf232;
mod codec;
use polynomial::BinaryPolynomial;

fn count_ones(mut i: u64) -> u8 {
    let mut result = 0;
    while i > 0 {
        if i & 1 == 1 {
            result += 1;
        }
        i >>= 1;
    }
    result
}

fn main() {
    let mut irreducibles = vec![
        BinaryPolynomial(0b10),
        BinaryPolynomial(0b11),
        BinaryPolynomial(0b111),
    ];
    'outer: for i in 0b1000..0b10000000000000000 {
        if i % 2 == 0 {
            continue;
        }
        if count_ones(i) % 2 == 0 {
            continue;
        }
        let x = BinaryPolynomial(i);
        for y in irreducibles.iter() {
            if y.degree() > (x.degree() + 1) / 2 {
                break;
            }
            if x % *y == BinaryPolynomial(0) {
                continue 'outer;
            }
        }
        irreducibles.push(x);
    }
    'outer2: for i in 1 << 32..1 << 33 {
        if i % 2 == 0 {
            continue;
        }
        if count_ones(i) % 2 == 0 {
            continue;
        }
        let x = BinaryPolynomial(i);
        for y in irreducibles.iter() {
            if x % *y == BinaryPolynomial(0) {
                continue 'outer2;
            }
        }
        println!("Degree 32 irreducible: {:?}", x);
        break;
    }
    //for x in irreducibles {
    //    println!("{:?}", x);
    //}
}
