use std::{str::FromStr, str, fmt::{Display, self}};
use crate::{Error, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType {
    pub chunk: [u8;4]
}

impl ChunkType {
    pub fn bytes(&self) -> [u8;4] {
        self.chunk
    }
    pub fn is_valid(&self) -> bool {
        if self.chunk[2].is_ascii_uppercase() {
            true
        }
        else{
            false
        }

    }
    pub fn is_critical(&self) -> bool {
        if self.chunk[0].is_ascii_uppercase() {
            true
        }
        else{
            false
        }
    }
    pub fn is_public(&self) -> bool {
        if self.chunk[1].is_ascii_uppercase() {
            true
        }
        else {
            false
        }
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        if self.chunk[2].is_ascii_uppercase() {
            true
        }
        else{
            false
        }
    }
    pub fn is_safe_to_copy(&self) -> bool {
        if self.chunk[3].is_ascii_uppercase() {
           false 
        }
        else {
           true 
        }
    }
}

impl TryFrom<[u8;4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8;4]) -> Result<Self> {
        Ok(ChunkType{chunk: value})
    }

}

impl FromStr for ChunkType {
    fn from_str(value: &str) -> Result<Self> {
        if value.chars().all(|c| c.is_alphabetic()) {
            let bytes = value.as_bytes(); 
            Ok(ChunkType{chunk: bytes.try_into().unwrap()})
        }
        else {
            Err("Found digits".into())
        }

    }

    type Err = Error;

}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", str::from_utf8(&self.chunk).unwrap())
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