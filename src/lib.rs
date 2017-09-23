#[cfg(test)]
extern crate rand;

mod polynomial;
mod gf232;
mod polygf232;
mod codec;

pub use codec::encode;
pub use codec::decode;
