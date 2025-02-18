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
    // Create data directory before opening connection
    if let Some(parent) = Path::new(DB_PATH).parent() {
        fs::create_dir_all(parent)?;
    }

    println!("Opening database connection...");
    let conn = Connection::open(DB_PATH).await?;
    
    println!("Initializing database...");
    init_db(&conn).await?;

    // let config = ConfigBuilder::default().use_custom_parameters(tfhe::shortint::parameters::V0_11_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64,).build();
    // let (client_key, _server_key) = generate_keys(config);

    // let public_key = CompactPublicKey::new(&client_key);
    // let compact_lista = CompactCiphertextList::builder(&public_key).push(25u8).build();
    // let compact_listb = CompactCiphertextList::builder(&public_key).push(7u8).build();
    // let expandeda = compact_lista.expand().unwrap();
    // let expandedb = compact_listb.expand().unwrap();
    // let a: FheUint8 = expandeda.get(0).unwrap().unwrap();
    // let b: FheUint8 = expandedb.get(0).unwrap().unwrap();

    // println!("Serializing...");
    // let mut serialized_data: Vec<u8> = vec![];
    // bincode::serialize_into(&mut serialized_data, &_server_key)?;
    // bincode::serialize_into(&mut serialized_data, &a)?;
    // bincode::serialize_into(&mut serialized_data, &b)?;
    
    // let mut serialized_data = Cursor::new(serialized_data);
    // let server_key: ServerKey = bincode::deserialize_from(&mut serialized_data)?;
    // let ct_1: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;
    // let ct_2: FheUint8 = bincode::deserialize_from(&mut serialized_data)?;

    // set_server_key(_server_key);
    // let result = ct_1 + ct_2;
    // let serialized_result = bincode::serialize(&result)?;
    // let result: FheUint8 = bincode::deserialize(&serialized_result)?;
    // let output: u8 = result.decrypt(&client_key);
    // assert_eq!(output, 32u8);
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
    // Serialize the ciphertext
    let serialized_ciphertext = bincode::serialize(ciphertext)?;
    
    // Convert U256 to bytes (fixed size array of 32 bytes)
    let mut key_bytes = [0u8; 32];
    key.to_big_endian(&mut key_bytes);
    
    conn.call(move |conn| {
        conn.execute(
            "INSERT OR REPLACE INTO computations (key, ciphertext) VALUES (?1, ?2)",
            rusqlite::params![key_bytes.to_vec(), serialized_ciphertext],
        )
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