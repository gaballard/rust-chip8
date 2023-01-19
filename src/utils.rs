///
/// Returns an individual bit from a given byte
/// 
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
