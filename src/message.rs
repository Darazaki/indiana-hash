use crate::hashing_algorithm::HashingAlgorithm;

#[derive(Debug, Clone)]
pub enum Message {
    HashingAlgorithmSelected(HashingAlgorithm),
    FilePathChanged(String),
}
