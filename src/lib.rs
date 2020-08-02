pub fn calculate_hash(
    pre_hash: &String,
    transactions: &Vec<Transaction>,
    timestamp: i64,
) -> String {
    let mut bytes = vec![];
    bytes.extend(&timestamp.to_ne_bytes());
    bytes.extend(
        transactions
            .iter()
            .flat_map(|transaction| transaction.bytes())
            .collect::<Vec<u8>>(),
    );
    bytes.extend(pre_hash.as_bytes());

    blake3::hash(&bytes).to_hex().to_string()
}