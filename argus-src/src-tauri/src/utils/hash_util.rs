use sha2::{Digest, Sha256};

/// 计算字符串的 SHA-256 哈希值
pub fn sha256(input: &str) -> String {
    format!("{:x}", Sha256::digest(input.as_bytes()))
}

/// 计算字节数组的 SHA-256 哈希值
pub fn sha256_bytes(input: &[u8]) -> String {
    format!("{:x}", Sha256::digest(input))
}


#[test]
fn test_sha256() {
    let input = "hello";
    let hash = sha256(input);
    println!("SHA256: {}", hash);

    let bytes_hash = sha256_bytes(&[1, 2, 3, 4]);
    println!("SHA256 (bytes): {}", bytes_hash);

    println!("{}", sha256("sha256"))
}
