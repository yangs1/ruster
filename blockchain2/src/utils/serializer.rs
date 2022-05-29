use serde::{Serialize, Deserialize};
use crypto::{digest::Digest, sha3::Sha3};
use crate::error::BlockchainError;

/* 序列化数据 */
pub fn serialize<T>(data: &T) -> Result<Vec<u8>, BlockchainError>
where
    T: ?Sized + Serialize,
{
    Ok(bincode::serialize(data)?)
}

/* 反序列化数据 */
pub fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, BlockchainError> 
where
    T: Deserialize<'a> + ?Sized
{
    Ok(bincode::deserialize(data)?)
}

/* 将序列化字符串 转成 hash字符串*/
pub fn hash_to_string(data : &[u8]) -> String
{
    let mut hasher = Sha3::sha3_256();

    hasher.input(data);
    
    hasher.result_str()
}

pub fn hash_to_u8(data: &[u8], out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result(out);
}