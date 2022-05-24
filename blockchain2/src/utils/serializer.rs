use serde::{Serialize, Deserialize};
use crypto::{digest::Digest, sha3::Sha3};
use crate::error::BlockchainError;

pub fn serialize<T>(data: &T) -> Result<Vec<u8>, BlockchainError>
where
    T: ?Sized + Serialize,
{
    Ok(bincode::serialize(data)?)
}


pub fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, BlockchainError> 
where
    T: Deserialize<'a> + ?Sized
{
    Ok(bincode::deserialize(data)?)
}


pub fn hash_to_string(data : &[u8]) -> String
{
    let mut hasher = Sha3::sha3_256();

    hasher.input(data);
    
    hasher.result_str()
}