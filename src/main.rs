use tfhe::prelude::*;
use tfhe::{
    generate_keys, set_server_key, CompactCiphertextList, CompactPublicKey, ConfigBuilder, FheUint8, ServerKey
};
use std::io::Cursor;
use axum::{
    routing::{get, post}, Router, Json, extract::State,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use tokio_rusqlite::Connection;
use primitive_types::U256;
use std::path::Path;
use std::fs;
mod keys;


const DB_PATH: &str = "data/tfhe.db";

fn single_encryption(public_key: &CompactPublicKey, a: u8) -> Result<(), Box<dyn std::error::Error>> {
    let public_key = keys::load_public_key()?;
    let compact_list = CompactCiphertextList::builder(&public_key).push(a).build();
    let _expanded = compact_list.expand().unwrap();
    let value:FheUint8 = _expanded.get(0).unwrap().unwrap();
    let mut serialized_data: Vec<u8> = vec![];
    bincode::serialize_into(&mut serialized_data, &value)?;
    println!("serialized_data: {:?}", serialized_data);
    // add logic to write to db
    Ok(())
}

fn add(lhs: U256, rhs: U256) {
    // pull the ciphertexts from the db
    //initialize the server w the key
    // add the ciphertexts
    // serialize the result
    // write the result to the db
}

fn requestDecryption(ct: U256) {
    
}
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    if let Some(parent) = Path::new(DB_PATH).parent() {
        fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(DB_PATH).await?;
    init_db(&conn).await?;

    println!("Loading public key and creating test value...");
    let public_key = keys::load_public_key()?;
    let compact_list = CompactCiphertextList::builder(&public_key).push(25u8).build();
    let _expanded = compact_list.expand().unwrap();
    let value:FheUint8 = _expanded.get(0).unwrap().unwrap();

    println!("Inserting test value with key 1234...");
    insert_computation(&conn, U256::from(123400), &value).await?;

    // Verify the data was written
    println!("Verifying data was written...");
    if let Some(_) = get_computation(&conn, U256::from(1234)).await? {
        println!("✅ Data successfully written and retrieved!");
    } else {
        println!("❌ Data not found in database!");
    }

    // Add this to your main function after insert:
    dump_database(&conn).await?;

    Ok(())  
}


async fn init_db(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure data directory exists with proper permissions
    if let Some(parent) = Path::new(DB_PATH).parent() {
        println!("Creating directory at: {:?}", parent);
        fs::create_dir_all(parent)?;
    }

    println!("Attempting to open database at: {}", DB_PATH);
    
    // Create the database connection first
    conn.call(|conn| {
        println!("Creating table...");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS computations (
                key BLOB NOT NULL PRIMARY KEY,
                ciphertext BLOB NOT NULL
            )",
            [],
        )
    }).await?;

    println!("Database initialized successfully");
    Ok(())
}

async fn insert_computation(
    conn: &Connection, 
    key: U256, 
    ciphertext: &FheUint8
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Serializing ciphertext...");
    let serialized_ciphertext = bincode::serialize(ciphertext)?;
    println!("Serialized ciphertext size: {} bytes", serialized_ciphertext.len());
    
    println!("Converting U256 key to bytes...");
    let mut key_bytes = [0u8; 32];
    key.to_big_endian(&mut key_bytes);
    println!("Key bytes: {:?}", key_bytes.to_vec());
    
    println!("Executing INSERT query...");
    let result = conn.call(move |conn| {
        // Start a transaction
        let tx = conn.transaction()?;
        
        tx.execute(
            "INSERT OR REPLACE INTO computations (key, ciphertext) VALUES (?1, ?2)",
            rusqlite::params![key_bytes.to_vec(), serialized_ciphertext],
        )?;
        
        // Commit the transaction
        tx.commit()?;
        Ok(1) // Return number of affected rows
    }).await?;
    
    println!("Insert completed. Rows affected: {}", result);

    // Verify the data immediately after insert
    conn.call(move |conn| {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM computations",
            [],
            |row| row.get(0),
        )?;
        println!("Total rows in database: {}", count);
        Ok(())
    }).await?;

    Ok(())
}

// Helper function to retrieve data
async fn get_computation(
    conn: &Connection,
    key: U256,
) -> Result<Option<FheUint8>, Box<dyn std::error::Error>> {
    // Convert U256 to bytes
    let mut key_bytes = [0u8; 32];
    key.to_big_endian(&mut key_bytes);

    let result = conn.call(move |conn| {
        conn.query_row(
            "SELECT ciphertext FROM computations WHERE key = ?1",
            rusqlite::params![key_bytes.to_vec()],
            |row| row.get::<_, Vec<u8>>(0)
        )
    }).await;

    match result {
        Ok(bytes) => {
            let ciphertext: FheUint8 = bincode::deserialize(&bytes)?;
            Ok(Some(ciphertext))
        }
        Err(tokio_rusqlite::Error::Rusqlite(rusqlite::Error::QueryReturnedNoRows)) => Ok(None),
        Err(e) => Err(Box::new(e)),
    }
}

async fn example_usage() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(DB_PATH).await?;
    init_db(&conn).await?;

    // // Example key and ciphertext
    // let key = U256::from(1234);  // Your U256 key
    // let ciphertext = /* your FheUint8 */;

    // // Insert
    // insert_computation(&conn, key, &ciphertext).await?;

    // // Retrieve
    // if let Some(retrieved_ciphertext) = get_computation(&conn, key).await? {
    //     println!("Retrieved ciphertext for key: {}", key);
    // }

    Ok(())
}

async fn dump_database(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nDumping database contents:");
    conn.call(|conn| {
        let mut stmt = conn.prepare("SELECT * FROM computations")?;
        let rows = stmt.query_map([], |row| {
            let key: Vec<u8> = row.get(0)?;
            let cipher: Vec<u8> = row.get(1)?;
            Ok((key, cipher))
        })?;
        
        for row in rows {
            if let Ok((key, cipher)) = row {
                println!("Key length: {}, Cipher length: {}", key.len(), cipher.len());
            }
        }
        Ok(())
    }).await?;
    Ok(())
}