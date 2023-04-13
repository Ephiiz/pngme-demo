use crc::{Crc, CRC_32_ISO_HDLC};
use std::{
    fmt::Display,
    io::{BufReader, Read},
};

use crate::chunk_type::ChunkType;

#[derive(Debug)]
pub struct Chunk {
    data_length: u32, // The length of only the data field, should not exceed 2^31
    chunk_type: ChunkType,
    message_bytes: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        //First ensure validity
        let mut reader = BufReader::new(value);
        let data_length: u32 = u32::from_be_bytes({
            let mut buf = [0; 4];
            reader.read_exact(&mut buf)?;
            buf
        });
        let chunk_type: ChunkType = {
            let mut buf = [0; 4];
            reader.read_exact(&mut buf)?;
            ChunkType::try_from(buf)?
        };
        let message_bytes: Vec<u8> = {
            let mut message: Vec<u8> = Vec::new();
            for _ in 0..data_length {
                let mut buf = [0; 1];
                reader.read_exact(&mut buf)?;
                message.push(buf[0]);
            }
            message
        };
        let crc: u32 = u32::from_be_bytes({
            let mut buf = [0; 4];
            reader.read_exact(&mut buf)?;
            buf
        });

        let p = Chunk {
            data_length,
            chunk_type,
            message_bytes,
            crc,
        };

        if !p.is_valid() {
            return Err("invalid Chunk".into());
        }
        Ok(p)
    }
}
impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.data_as_string().unwrap())
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let mut p = Chunk {
            data_length: data.len() as u32,
            chunk_type: chunk_type,
            message_bytes: data,
            crc: 0,
        };
        p.crc = p.crc();
        p
    }
    pub fn is_valid(&self) -> bool {
        let p: bool = self.chunk_type.is_valid()
            && (self.data_length == self.message_bytes.len() as u32)
            && (self.crc == self.crc());
        p
    }
    pub fn length(&self) -> u32 {
        self.data_length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.message_bytes
    }
    pub fn crc(&self) -> u32 {
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut digest = crc.digest();
        digest.update(&self.chunk_type.bytes());
        digest.update(&self.message_bytes.clone().as_slice());
        digest.finalize()
    }
    pub fn data_as_string(&self) -> crate::Result<String> {
        let mut out = String::new();
        for byte in &self.message_bytes {
            out.push(*byte as char);
        }
        Ok(out)
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        // Naive method, but it works i guess
        let mut out: Vec<u8> = self.data_length.to_be_bytes().iter().copied().collect();
        for byte in &self.chunk_type.bytes() {
            out.push(*byte);
        }
        for byte in &self.message_bytes {
            out.push(*byte);
        }
        for byte in &self.crc.to_be_bytes() {
            out.push(*byte);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
