use blockchainlib::*;

fn main() {
    let base: u128 = 2;
    let exponent = 110;
    let mut block = Block::new(0, 0, vec![0; 32], 0, "Genesis block!".to_owned(), base.pow(exponent));

    block.mine();
    
    println!("{:?}", &block);

}