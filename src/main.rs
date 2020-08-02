fn main() {
    let mut blockchain = Blockchain::new();

    let genesis_block = Block::new(
        "0".to_owned(),
        vec![Transaction {
            sender: String::from("Naman"),
            receiver: String::from("Mihir"),
            amount: 2000.0,
        }],
    );

    let first_block = Block::new(
        genesis_block.hash.to_owned(),
        vec![Transaction {
            sender: String::from("Mihir"),
            receiver: String::from("Gulfam"),
            amount: 2500.0,
        }],
    );

    let second_block = Block::new(
        first_block.hash.to_owned(),
        vec![Transaction {
            sender: String::from("Michal"),
            receiver: String::from("Dan"),
            amount: 1000.0,
        }],
    );

    blockchain.add_block(genesis_block);
    blockchain.add_block(first_block);
    blockchain.add_block(second_block);

    println!("{:#?}", blockchain);
}