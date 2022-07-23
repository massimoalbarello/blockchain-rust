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

    pub fn verify_last_block (&self, new_block_index: u32, new_block_timestamp: u128) -> bool {
        let last_block = &self.blocks[self.blocks.len()-1];
        if last_block.index != new_block_index-1 {
            println!("Not adjacent indeces");
            return false;
        }
        else if !last_block.check_difficulty(&last_block.hash()) {
            println!("Hash doesn't match the difficulty");
            return false;
        }
        else if last_block.index != 0 {
            if last_block.timestamp > new_block_timestamp {
                println!("Time did not increase");
                return false;
            }
            else if last_block.prev_block_hash != self.blocks[self.blocks.len()-2].hash {
                println!("Last block has invalid prev_block_hash");
                return false;
            }
        }
        else {
            // genesis block
            if last_block.prev_block_hash != vec![0; 32] {
                println!("Genesis block has invalid prev_block_hash"); 
                return false;
            }
        }
         
        true
    }
}