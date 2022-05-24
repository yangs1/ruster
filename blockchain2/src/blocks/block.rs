use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::utils::serializer::{serialize, hash_to_string};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    timestamp: i64,   // 时间戳
    prev_hash: String, // 前一个区块的Hash值
    nonce: usize,    // 随机数，用于计算工作量证明
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    header: BlockHeader, // 区块头
    data: String,        // 区块存储的数据，后面在实现交易的功能时，这个字段会修改为交易集合。
    hash: String,        // 块的Hash值
}


impl Block{
    pub fn new(data: &str, prev_hash : &str) -> Block{
        let header = BlockHeader{
            timestamp : Utc::now().timestamp(),
            prev_hash: prev_hash.into(), 
            nonce: 0, 
        };
        
        let mut block = Block { header: header, data: data.into(), hash: String::new() };
        
        block.set_hash();

        block
    }

    // 将 Block.header 序列化， 序列化值写入 hash 中
    fn set_hash(&mut self){
        let header = serialize(&self.header).unwrap();
        self.hash = hash_to_string(&header);
    }

    pub fn get_hash(&self) -> String
    {
        self.hash.clone()
    }

    pub fn create_genesis_block() -> Block{
        Block::new("创世区块", &String::from(""))
    }
}