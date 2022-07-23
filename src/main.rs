use blockchainlib::*;

fn main() {
    let base: u128 = 2;
    let exponent = 120;
    let difficulty: u128 = base.pow(exponent);
    let mut block = Block::new(0, now(), vec![0; 32], 0, "Genesis block!".to_owned(), difficulty);
    
    block.mine();
    println!("Mined geneesis block: {:?}", &block);

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    for block_index in 1..=10 {
        let current_timestamp = now();

        if blockchain.verify_last_block(block_index, current_timestamp) {
            let last_block_hash = blockchain.get_last_block_hash();
            let mut block = Block::new(block_index, current_timestamp, last_block_hash, 0, format!("Block {}!", block_index).to_owned(), difficulty);
            
            block.mine();
            println!("Mined new block: {:?}", &block);

            blockchain.append_block(block);
        }
        else {
            break;
        }
        
    }
}