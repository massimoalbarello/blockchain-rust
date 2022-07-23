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

    for i in 1..=10 {
        let last_block_hash = blockchain.get_last_block_hash();
        let mut block = Block::new(i, now(), last_block_hash, 0, format!("Block {}!", i).to_owned(), difficulty);
        block.mine();
        println!("Mined new block: {:?}", &block);

        blockchain.append_block(block);
    }
}