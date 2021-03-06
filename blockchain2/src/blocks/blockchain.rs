use crate::blocks::block::Block;
use tracing::info;


const CURR_BITS: usize = 8;

#[derive(Debug)]
pub struct Blockchain{
    blocks : Vec<Block>,
    height : usize,
}



impl Blockchain{
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::create_genesis_block(CURR_BITS)],
            height: 0,
        }
    }

    pub fn mine_block(&mut self, data:&str) {
        let last_block = self.blocks.last().unwrap();

        let block = Block::new(data, last_block.get_hash().as_str(), CURR_BITS);

        self.blocks.push(block);

        self.height += 1;
    }

    pub fn blocks_info(&self) {
        for block in self.blocks.iter() {
            info!("{:#?}", block);
        }
    }
}