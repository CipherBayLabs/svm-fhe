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

async fn single_encryption(conn: &Connection, public_key: &CompactPublicKey, a: u8) -> Result<(), Box<dyn std::error::Error>> {
    let public_key = keys::load_public_key()?;
    let compact_list = CompactCiphertextList::builder(&public_key).push(a).build();
    let _expanded = compact_list.expand().unwrap();
    let value:FheUint8 = _expanded.get(0).unwrap().unwrap();
    let mut serialized_data: Vec<u8> = vec![];
    bincode::serialize_into(&mut serialized_data, &value)?;
    println!("serialized_data: {:?}", serialized_data);
    let public_key = keys::load_public_key()?;
    let key = string_to_u256("9999999999999999999999999999999999999")?;
    let compact_list = CompactCiphertextList::builder(&public_key).push(26u8).build();
    let _expanded = compact_list.expand().unwrap();
    let value:FheUint8 = _expanded.get(0).unwrap().unwrap();
    insert_computation(&conn, key, &value).await?;
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

fn string_to_u256(hex_str: &str) -> Result<U256, Box<dyn std::error::Error>> {
    if hex_str.starts_with("0x") {
        Ok(U256::from_str_radix(&hex_str[2..], 16)?)
    } else {
        Ok(U256::from_dec_str(hex_str)?)
    }
}

// Add these route handlers
async fn hello() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize)]
struct ComputeRequest {
    value: u8,
}

#[derive(Serialize)]
struct ComputeResponse {
    result: String,
}

async fn compute(Json(payload): Json<ComputeRequest>) -> Json<ComputeResponse> {
    Json(ComputeResponse {
        result: format!("Computed value: {}", payload.value)
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = Path::new(DB_PATH).parent() {
        fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(DB_PATH).await?;
    init_db(&conn).await?;
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/", get(hello)) 
        .route("/compute", post(compute))
        .layer(cors)
        .with_state(conn);
    println!("Server running on http://localhost:3000");
    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000").await?,
        app
    ).await?;
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
) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
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
        Ok(bytes) => Ok(Some(bytes)),
        Err(tokio_rusqlite::Error::Rusqlite(rusqlite::Error::QueryReturnedNoRows)) => Ok(None),
        Err(e) => Err(Box::new(e)),
    }
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