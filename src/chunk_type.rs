// u32::from_be_bytes([1,2,3,4]);
// is_ascii_lowercase
// is_ascii_uppercase
// is_ascii
// std::as_bytes

use std::{
    fmt::Display,
    str::{from_utf8, FromStr},
};

#[derive(PartialEq, Eq, Debug)]
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

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in value {
            if !byte.is_ascii_alphabetic() {
                return Err("invalid ChunkType".into());
            }
        }
        let t = ChunkType { value };
        Ok(t)
    }
}

impl FromStr for ChunkType {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arr: [u8; 4] = {
            let mut p: [u8; 4] = [0; 4];
            let mut i = 0;
            for char in s.bytes() {
                if char.is_ascii_alphabetic() {
                    p[i] = char;
                    i += 1;
                } else {
                    return Err("invalid ChunkType".into());
                }
            }
            p
        };
        Ok(ChunkType { value: arr })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", from_utf8(&self.value).unwrap())
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.value
    }

    pub fn is_valid(&self) -> bool {
        //TODO
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        self.value[0].is_ascii_uppercase()
    }
    pub fn is_public(&self) -> bool {
        self.value[1].is_ascii_uppercase()
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.value[2].is_ascii_uppercase()
    }
    pub fn is_safe_to_copy(&self) -> bool {
        self.value[3].is_ascii_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
