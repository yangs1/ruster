use chrono::Utc;
use serde::{Serialize, Deserialize};
use utils::coder;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}

impl Block {
    fn set_hash(&mut self) {
        self.header.time = Utc::now().timestamp();
        let hasher = coder::my_serialize(&(self.header.tx_hash));

        self.hash = coder::get_hash(&hasher[..]);
    }

    pub fn new(data: String, pre_hash: String) -> Block {
        let transactions = coder::my_serialize(&data);
        let tx_hash = coder::get_hash(&transactions[..]);

        let time = Utc::now().timestamp();

        let mut block = Block {
            header: BlockHeader {
                time,
                tx_hash,
                pre_hash,
            },
            hash: "".to_string(),
            data,
        };
        
        block.set_hash();

        block
    }
}
