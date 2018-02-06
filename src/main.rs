mod block;
use block::Block;

fn main() {
    let block = Block::new(1, 2, 4, 2);
    println!("{}", block);
}
