#![allow(dead_code)]
use std::{fmt, str::FromStr};

use anyhow::{ensure, Ok};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType(u32);

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        let b1: u8 = ((self.0 >> 24) & 0xff) as u8;
        let b2: u8 = ((self.0 >> 16) & 0xff) as u8;
        let b3: u8 = ((self.0 >> 8) & 0xff) as u8;
        let b4: u8 = (self.0 & 0xff) as u8;
        return [b1, b2, b3, b4];
    }
    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
    fn is_critical(&self) -> bool {
        self.0 & (0x20 << 24) == 0
    }
    fn is_public(&self) -> bool {
        self.0 & (0x20 << 16) == 0
    }
    fn is_reserved_bit_valid(&self) -> bool {
        self.0 & (0x20 << 8) == 0
    }
    fn is_safe_to_copy(&self) -> bool {
        self.0 & 0x20 != 0 // or self.0 & 0x20 == 0x20
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = anyhow::Error;

    fn try_from(source: [u8; 4]) -> Result<Self, Self::Error> {
        ensure!(source.into_iter().all(|x| x.is_ascii_alphabetic()));
        let [b1, b2, b3, b4] = source;
        Ok(ChunkType(u32::from_be_bytes([b1, b2, b3, b4])))
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        ensure!(bytes.len() == 4);

        let mut result: [u8; 4] = Default::default();
        for i in 0..4usize {
            let val = bytes[i];
            result[i] = val;
        }

        result.try_into()
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d] = self.bytes();
        write!(f, "{}{}{}{}", a as char, b as char, c as char, d as char)
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
