use crate::block;

pub struct BlockChain{
    pub blocks: Vec<block::Block>,
}

impl BlockChain{
    pub fn add_block(&mut self, data : String){
        let pre_block = &self.blocks[self.blocks.len() - 1];

        let new_block = block::Block::new(data, pre_block.hash.clone());
        
        self.blocks.push(new_block);
    }
    // 获取初始链表
    pub fn new_genesis_block() -> block::Block{
        block::Block::new("This is genesis block".to_string(), String::from(""))
    }

    // 初始化，链表
    // 调用自身函数是，不能想公共函数那样直接写 new_genesis_block， 需要带上 结构体名称(类名？) ： BlockChain
    pub fn new_blockchain() -> BlockChain{
        BlockChain{
            blocks: vec![BlockChain::new_genesis_block()],
        }
    }
}