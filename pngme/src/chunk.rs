#![allow(unused_variables)]
//mod chunk_type;
use crate::chunk_type::ChunkType;

use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::write;
//use crc::{crc32, Hasher32};
use crc::{Crc, CRC_32_ISO_HDLC};

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type:ChunkType,
    data: Vec<u8>,
    crc:u32
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {:?}, {}", self.length, self.chunk_type, self.data,self.crc )
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        //let mut length: [u8; 4] = Default::default();
        let data_length = data.len();
        if data_length < 12 {
            return Err("Invalid chunk data")
        }
        
        let mut chunk_type_temp: [u8; 4] = Default::default();
        let mut chunk_data: Vec<u8> = Default::default();
        
        
        //length.copy_from_slice(&data[..4]);
        chunk_type_temp.copy_from_slice(&data[4..8]);
        
        chunk_data = data[8..data_length - 4].to_vec();

        // let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        // let mut digest = crc.digest();
        // digest.update(&chunk_type_temp);
        // digest.update(&chunk_data);
        // let final_crc = digest.finalize();
        let given_crc = &data[data_length-4..];
        let calculated_crc = Chunk::calculate_crc(&chunk_type_temp, &chunk_data);

        if given_crc != calculated_crc.to_be_bytes() {
            return Err("Invalid CRC");
        }

        Ok(Chunk { 
            length:u32::from_be_bytes(data[..4].try_into().unwrap()),
            chunk_type:ChunkType::try_from(chunk_type_temp)?,
            data:chunk_data,
            crc:calculated_crc
         })

    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data:Vec<u8>) -> Chunk{
        let length = data.len() as u32;

        let crc = Chunk::calculate_crc(&chunk_type.bytes(), &data);

        Chunk { length: length, chunk_type: chunk_type, data: data, crc: crc }
    }

    pub fn calculate_crc(chunk_type: &[u8;4], data: &Vec<u8> ) -> u32{
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut digest = crc.digest();
        digest.update(chunk_type);
        digest.update(data);
        digest.finalize()
    }

    pub fn length(&self) -> u32{
        self.length
    }

    pub fn crc(&self) -> u32{
        self.crc
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data_as_string(&self) -> Result<String, std::str::Utf8Error>{
        match std::str::from_utf8(&self.data){
            Ok(string_data) => Ok(string_data.to_string()),
            Err(e) => Err(e)
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Convert length to bytes and append
        bytes.extend(&self.length.to_be_bytes());

        // Get the bytes of chunk_type and append
        bytes.extend(&self.chunk_type.bytes());

        // Append the data bytes
        bytes.extend(&self.data);

        // Convert crc to bytes and append
        bytes.extend(&self.crc.to_be_bytes());

        bytes
    }
}

fn main() {
    #[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::{println, str::FromStr};

    fn testing_chunk() -> Chunk{
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
        let data = "This is where your secret message will be!".as_bytes().to_vec();
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

        let chunk: Result<Chunk, &str> = Chunk::try_from(chunk_data.as_ref());

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


}


