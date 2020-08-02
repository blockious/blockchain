#[derive(Debug)]
pub struct Block {
    pub timestamp: i64,
    pub hash: String,
    pub pre_hash: String,
    pub transaction: Vec<Transaction>,
}