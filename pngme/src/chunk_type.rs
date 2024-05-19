
#![allow(unused_variables)]
use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt;


#[derive(Debug)]
#[derive(PartialEq)]
pub struct ChunkType {
    b1: u8,
    b2: u8,
    b3: u8,
    b4: u8
}

impl ChunkType{
    pub fn bytes(&self) -> [u8;4]{
        [
            self.b1,
            self.b2,
            self.b3,
            self.b4
        ]
    }

    pub fn to_string(&self) -> String{
        let bytes: &[u8] = &[self.b1, self.b2, self.b3, self.b4];
        match std::str::from_utf8(bytes) {
            Ok(val) => val.to_string(),
            Err(e) => {
                String::from("")
            }
        }
    }

    pub fn is_critical(&self) -> bool{
        (self.b1 & 0b0010_0000) == 0
    }

    pub fn is_public(&self) -> bool{
        (self.b2 & 0b0010_0000) == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool{
        (self.b3 & 0b0010_0000) == 0
    }

    pub fn is_safe_to_copy(&self) -> bool{
        (self.b4 & 0b0010_0000) != 0
    }

    pub fn is_valid(&self) -> bool {
        self.is_critical() && self.is_public() && self.is_reserved_bit_valid() && self.is_safe_to_copy()
    }
}

impl TryFrom<[u8;4]> for ChunkType{
    type Error = &'static str;
    fn try_from(data:[u8;4]) -> Result<Self, Self::Error>{
        Ok(ChunkType{
            b1: data[0],
            b2: data[1],
            b3: data[2],
            b4: data[3],
        })
    }
}

impl fmt::Display for ChunkType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.b1, self.b2,self.b3,self.b4)
    }
}

impl FromStr for ChunkType{
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars:Vec<u8> = s.bytes().collect();
        Ok(ChunkType{
            b1:chars[0],
            b2:chars[1],
            b3:chars[2],
            b4:chars[3],
        })

    }

}

fn main() {
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

}