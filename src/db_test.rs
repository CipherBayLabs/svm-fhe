use tokio_rusqlite::Connection;
use std::path::Path;
use tfhe::prelude::*;
use tfhe::{CompactCiphertextList, ClientKey, FheUint64};
mod keys;

const DB_PATH: &str = "data/tfhe.db";

pub async fn test_first_value() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting database test...");

    let client_key = keys::load_client_key()?;
    let conn = Connection::open(DB_PATH).await?;
    let blob = conn.call(|conn| {
        let mut stmt = conn.prepare("SELECT ciphertext FROM computations LIMIT 1")?;
        let blob: Vec<u8> = stmt.query_row([], |row| row.get(0))?;
        Ok(blob)
    }).await?;

    println!("Retrieved blob of size: {} bytes", blob.len());
    let ciphertext: CompactCiphertextList = bincode::deserialize(&blob)?;
    let expanded = ciphertext.expand()?;
    let value: u64 = expanded.get::<FheUint64>(0)?.unwrap().decrypt(&client_key);
    
    println!("Decrypted value: {}", value);
    assert_eq!(value, 1000000000);
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_first_value().await
    
}