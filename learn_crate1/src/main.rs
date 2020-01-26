use crypto::digest::Digest;
use crypto::sha3::Sha3;

fn main() {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str("Hello world!");
    let result = hasher.result_str();
    println!("result: {}", result);
}
