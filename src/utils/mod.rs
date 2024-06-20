mod compiler;
mod decompiler;

use std::{convert::TryInto, fmt::Debug};

pub use self::compiler::*;
pub use self::decompiler::*;

///
/// Returns an individual bit from a given byte
///
#[inline]
pub fn read_bit_from_byte(byte: &u8, bit_position: u8) -> &u8 {
    if bit_position < 8 {
        if byte & (1 << bit_position) != 0 {
            &1
        } else {
            &0
        }
    } else {
        &0
    }
}

#[allow(dead_code)]
pub fn vec_to_array<T: Debug, const N: usize>(vec: Vec<T>) -> [T; N] {
    vec.try_into().unwrap()
}
