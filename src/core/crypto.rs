use sha2::{Sha256, Digest};
use md5::Md5;
use sha1::Sha1;

pub struct CryptoEngine;

impl CryptoEngine {
    pub fn generate_sha256(data: &[u8]) -> String { hex::encode(Sha256::digest(data)) }
    pub fn generate_md5(data: &[u8]) -> String { hex::encode(Md5::digest(data)) }
    pub fn generate_sha1(data: &[u8]) -> String { hex::encode(Sha1::digest(data)) }
}