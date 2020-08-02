#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>
}

impl Blockchain {
    pb fun new() -> Self {
        Blockchain {blocks: vec![]}
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block)
    }
}