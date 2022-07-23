use super::*;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn append_block (&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn get_last_block_hash (&self) -> BlockHash {
        self.blocks[self.blocks.len()-1].hash.clone()
    }
}