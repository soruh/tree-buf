mod compress;
pub mod delta;
mod dictionary;
pub mod packed_bool;
pub mod rle;
pub mod rle_bool;
pub mod varint;

use crate::prelude::*;
pub(crate) use compress::*;
pub(crate) use dictionary::*;
pub(crate) use rle::*;

mod gorilla_new;
mod gorilla_old;
pub mod gorilla {
    pub use super::gorilla_new::{compress, size_for};
    pub use super::gorilla_old::decompress;
}

//pub mod zfp;

#[cfg(feature = "decode")]
/// Decodes all items from some byte aligned encoding
pub fn decode_all<T>(bytes: &[u8], f: impl Fn(&[u8], &mut usize) -> DecodeResult<T>) -> DecodeResult<Vec<T>> {
    profile_fn!(decode_all);
    let mut offset = 0;
    let mut result = Vec::new();
    while offset < bytes.len() {
        let decode = f(bytes, &mut offset)?;
        result.push(decode);
    }
    debug_assert_eq!(offset, bytes.len());

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;
    #[cfg(all(feature = "decode", feature = "encode"))]
    pub fn round_trip<T: Copy + PartialEq + Debug>(data: &[T], encoder: impl Fn(T, &mut Vec<u8>), decoder: impl Fn(&[u8], &mut usize) -> DecodeResult<T>) -> DecodeResult<()> {
        let mut bytes = Vec::new();
        for value in data.iter() {
            encoder(*value, &mut bytes);
        }

        let result = decode_all(&bytes, decoder)?;

        assert_eq!(&result, &data);
        Ok(())
    }
}
