use tokio_rusqlite::Connection;
use std::path::Path;
use tfhe::prelude::*;
use tfhe::{CompactCiphertextList, ClientKey, FheUint64};
mod keys;

const DB_PATH: &str = "data/tfhe.db";

pub async fn test_first_value_zero() -> Result<(), Box<dyn std::error::Error>> {
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
    assert_eq!(value, 0);
    Ok(())
}

pub async fn test_last_values() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting last values test...");
    let client_key = keys::load_client_key()?;
    let conn = Connection::open(DB_PATH).await?;
    
    let blobs = conn.call(|conn| {
        let mut stmt = conn.prepare("SELECT ciphertext FROM computations ORDER BY ROWID DESC LIMIT 3")?;
        let blobs: Vec<Vec<u8>> = stmt.query_map([], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(blobs)
    }).await?;

    println!("Retrieved {} values", blobs.len());
    
    for (i, blob) in blobs.iter().enumerate() {
        let ciphertext: CompactCiphertextList = bincode::deserialize(&blob)?;
        let expanded = ciphertext.expand()?;
        let value: u64 = expanded.get::<FheUint64>(0)?.unwrap().decrypt(&client_key);
        println!("Value {}: {}", i + 1, value);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_first_value_zero().await;
    test_last_values().await
}