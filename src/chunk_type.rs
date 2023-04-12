// u32::from_be_bytes([1,2,3,4]);
// is_ascii_lowercase
// is_ascii_uppercase
// is_ascii
// std::as_bytes

use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Eq)]
pub struct ChunkType {
    // 4 bytes of ascii characters, to be treated as raw bytes, the 5th bit indicates capitalization, so caps can be used to read the 5th bit

    //Byte 1:
    // Ancillary Byte, if upper indicates critical, if lower indicates non-critical and can be ignored

    //Byte 2:
    // Private Byte, if upper indicates public, if lower indicates private

    //Byte 3:
    // Reserved Byte, must be uppercase, but should not complain about lowercase, just warn

    //Byte 4:
    // Safe to copy byte, if upper is unsafe to copy
    value: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {}

impl FromStr for ChunkType {}

impl Display for ChunkType {}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.value.clone()
    }

    pub fn is_valid(&self) -> bool {
        //TODO
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {}
    pub fn is_public(&self) -> bool {}
    pub fn is_reserved_bit_valid(&self) -> bool {}
    pub fn is_safe_to_copy(&self) -> bool {}
}
