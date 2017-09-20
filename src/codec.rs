use gf232::GF232;
use polygf232::PolyGF232;
use std::iter;

/// Converts a `u64` to a byte array (little-endian)
fn as_bytes(mut x: u64) -> Vec<u8> {
    let mut result = vec![];
    for _ in 0..8 {
        result.push((x & 0xFF) as u8);
        x >>= 8;
    }
    result
}

/// Converts an 8-byte array to a `u64` (little-endian)
fn as_u64(bytes: &[u8]) -> u64 {
    let mut result = 0;
    for i in 0..8 {
        result |= (bytes[i] as u64) << i * 8;
    }
    result
}

/// Struct iterating through an iterator returning bytes and converting them
/// into `u32`s on the fly (little-endian)
struct IterU32<I: Iterator<Item = u8>> {
    pub inner: I,
}

impl<I: Iterator<Item = u8>> Iterator for IterU32<I> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let mut result;
        if let Some(x) = self.inner.next() {
            result = x as u32;
        } else {
            return None;
        }
        if let Some(x) = self.inner.next() {
            result += (x as u32) << 8;
        }
        if let Some(x) = self.inner.next() {
            result += (x as u32) << 16;
        }
        if let Some(x) = self.inner.next() {
            result += (x as u32) << 24;
        }
        Some(result)
    }
}

/// Struct iterating through another iterator and returning `n` elements at a time
struct TakeN<I: Iterator> {
    n: usize,
    inner: I,
}

impl<I: Iterator> TakeN<I> {
    pub fn new(inner: I, n: usize) -> TakeN<I> {
        TakeN { n, inner }
    }
}

impl<I: Iterator> Iterator for TakeN<I> {
    type Item = Vec<<I as Iterator>::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = vec![];
        for _ in 0..self.n {
            if let Some(x) = self.inner.next() {
                result.push(x);
            } else {
                break;
            }
        }
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

/// Returns a polynomial interpolating the given points
fn interpolate(points: Vec<(usize, GF232)>) -> PolyGF232 {
    let mut result = PolyGF232::new(vec![]);
    let _x = PolyGF232::new(vec![GF232(0), GF232(1)]);
    for p in &points {
        let mut coefficient = PolyGF232::new(vec![GF232(1)]);
        for p2 in &points {
            if *p2 == *p {
                continue;
            }
            let l = GF232(p2.0 as u32);
            let j = GF232(p.0 as u32);
            coefficient = coefficient * (&_x + l) / (j + l);
        }
        result = result + coefficient * p.1;
    }
    result
}

/// Encodes a single set of `k` `u32`s
fn encode_stripe(data: &[u32], n: usize, k: usize) -> Vec<u32> {
    assert_eq!(data.len(), k);
    let interpolated = interpolate(data.into_iter().cloned().map(GF232).enumerate().collect());
    let mut result = vec![];
    for i in 0..n {
        let point = interpolated.apply(GF232(i as u32));
        result.push(point.0);
    }
    result
}

/// Decodes a single set of `k` `u32`s
fn decode_stripe(data: &[(usize, u32)], k: usize) -> Vec<u32> {
    assert!(data.len() >= k);
    let interpolated = interpolate(
        data.into_iter()
            .take(k)
            .cloned()
            .map(|(x, y)| (x, GF232(y)))
            .collect(),
    );
    let mut result = vec![];
    for i in 0..k {
        result.push(interpolated.apply(GF232(i as u32)).0);
    }
    result
}

/// Encodes a given array of bytes using striping
/// The length of the data is prepended to the array, and a padding of 0's is appended
/// in order to make sure that the data length is a multiple of `k` `u32`s.
pub fn encode(data: &[u8], n: usize, k: usize) -> Vec<Vec<u8>> {
    let stripe_size = k * 4;
    let padding = stripe_size - (8 + data.len()) % stripe_size;
    let length = 8 + data.len() + padding;
    let data_stream = IterU32 {
        inner: as_bytes(data.len() as u64)
            .into_iter()
            .chain(data.into_iter().cloned())
            .chain(iter::repeat(0))
            .take(length),
    };
    let stripes = TakeN::new(data_stream, k);
    let mut result = vec![];
    for _ in 0..n {
        result.push(vec![]);
    }
    for stripe in stripes {
        let encoded = encode_stripe(&stripe, n, k);
        for (v, val) in result.iter_mut().zip(encoded) {
            v.push((val & 0xFF) as u8);
            v.push(((val >> 8) & 0xFF) as u8);
            v.push(((val >> 16) & 0xFF) as u8);
            v.push(((val >> 24) & 0xFF) as u8);
        }
    }
    result
}

/// Struct iterating through multiple iterators simultaneously, returning
/// their items in a `Vec`
struct DecodeIter<I: Iterator<Item = u32>> {
    inner: Vec<(usize, I)>,
}

impl<I: Iterator<Item = u32>> DecodeIter<I> {
    pub fn new(inner: Vec<(usize, I)>) -> Self {
        DecodeIter { inner }
    }
}

impl<I: Iterator<Item = u32>> Iterator for DecodeIter<I> {
    type Item = Vec<(usize, u32)>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = vec![];
        for &mut (idx, ref mut itr) in &mut self.inner {
            if let Some(x) = itr.next() {
                result.push((idx, x));
            } else {
                return None;
            }
        }
        Some(result)
    }
}

/// Decodes `k` datasets into an array of bytes
pub fn decode(data: &[(usize, &[u8])], k: usize) -> Vec<u8> {
    assert!(data.len() >= k);
    let decode_iter = DecodeIter::new(
        data.into_iter()
            .map(|&(idx, data)| {
                (idx, IterU32 { inner: data.into_iter().cloned() })
            })
            .collect(),
    );
    let mut result = vec![];
    for stripe in decode_iter {
        let decoded = decode_stripe(&stripe, k);
        for x in decoded {
            result.push((x & 0xFF) as u8);
            result.push(((x >> 8) & 0xFF) as u8);
            result.push(((x >> 16) & 0xFF) as u8);
            result.push(((x >> 24) & 0xFF) as u8);
        }
    }
    let size = as_u64(&result[0..8]);
    result.into_iter().skip(8).take(size as usize).collect()
}

#[cfg(test)]
mod test {
    use rand::{self, Rng};
    use super::{encode, decode};

    #[test]
    fn test_encode_decode() {
        let mut rng = rand::thread_rng();
        let data: Vec<u8> = rng.gen_iter().take(10000).collect();
        let encoded = encode(&data, 10, 5);
        let mut indices = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        rng.shuffle(&mut indices);

        let decoded = decode(
            &[
                (indices[0], &encoded[indices[0]]),
                (indices[1], &encoded[indices[1]]),
                (indices[2], &encoded[indices[2]]),
                (indices[3], &encoded[indices[3]]),
                (indices[4], &encoded[indices[4]]),
            ],
            5,
        );
        assert_eq!(data, decoded);
    }
}
