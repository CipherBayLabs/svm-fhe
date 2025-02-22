use tfhe::prelude::*;
use tfhe::{
    set_server_key, CompactCiphertextList, CompactPublicKey, ConfigBuilder, FheUint8, ServerKey
};
use std::io::Cursor;
use axum::{
    routing::{get, post}, Router, Json, extract::State,
    http::StatusCode,
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

async fn single_encryption(key: [u8; 32], a: u8) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(DB_PATH).await?;
    let public_key = keys::load_public_key()?;
    let compact_list = CompactCiphertextList::builder(&public_key).push(a).build();
    let _expanded = compact_list.expand().unwrap();
    let value:FheUint8 = _expanded.get(0).unwrap().unwrap();
    let mut serialized_data: Vec<u8> = vec![];
    bincode::serialize_into(&mut serialized_data, &value)?;
    let public_key = keys::load_public_key()?;
    let compact_list = CompactCiphertextList::builder(&public_key).push(a).build();
    let _expanded = compact_list.expand().unwrap();
    let value:FheUint8 = _expanded.get(0).unwrap().unwrap();
    insert_computation(&conn, key, &value).await?;
    Ok(())
}

fn add(lhs: [u8; 32], rhs: [u8; 32]) {
    // pull the ciphertexts from the db
    //initialize the server w the key
    // add the ciphertexts
    // serialize the result
    // write the result to the db
}

// fn requestDecryption(ct: U256) {
    
// }

async fn hello() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize)]
struct ComputeRequest {
    value: u8,
}

#[derive(Deserialize)]
struct EncryptRequest {
    key: [u8; 32],
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

#[derive(Deserialize)]
struct InsertRequest {
    key: [u8; 32],
    value: FheUint8,
}

////////////////////////////////////////

#[derive(Deserialize)]
struct Request {
    value: u8,
}

#[derive(Serialize)]
struct Response {
    received: u8,
}

async fn handle_single_encryption(Json(payload): Json<Request>) -> Json<Response> {
    Json(Response {
        received: payload.value
    })
}

async fn handle_encrypt(Json(payload): Json<EncryptRequest>) -> StatusCode {
    match single_encryption(payload.key, payload.value).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = Path::new(DB_PATH).parent() {
        fs::create_dir_all(parent)?;
    }
    let conn = Connection::open(DB_PATH).await?;
    init_db(&conn).await?;

    // Add some test data
    let test_key = [1u8; 32]; 
    let test_value = 42u8;
    conn.call(move |conn| {
        conn.execute(
            "INSERT OR REPLACE INTO computations (key, ciphertext) VALUES (?1, ?2)",
            rusqlite::params![test_key.to_vec(), test_value.to_le_bytes()],
        )
    }).await?;

    dump_database(&conn).await?;

    let cors = CorsLayer::permissive();
    let app = Router::new()
        .route("/", get(hello))
        .route("/compute", post(compute))
        .route("/encrypt", post(handle_encrypt))
        .layer(cors)
        .with_state(conn.clone());

    println!("Server running on http://localhost:3000");
    
    // Could also add dump here to see initial state
    dump_database(&conn).await?;

    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000").await?,
        app
    ).await?;
    Ok(())
}


async fn init_db(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = Path::new(DB_PATH).parent() {
        println!("Creating directory at: {:?}", parent);
        fs::create_dir_all(parent)?;
    }
    println!("Attempting to open database at: {}", DB_PATH);    
    conn.call(|conn| {
        println!("Creating table...");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS computations (
                key CHAR(32) NOT NULL PRIMARY KEY,
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
    key: [u8; 32], 
    ciphertext: &FheUint8
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Serializing ciphertext...");
    let serialized_ciphertext = bincode::serialize(ciphertext)?;
    
    println!("Converting U256 key to bytes...");
    let mut key_bytes = [0u8; 32];  // Define key_bytes here    
    println!("Executing INSERT query...");
    let result = conn.call(move |conn| {
        let tx = conn.transaction()?;
        tx.execute(
            "INSERT OR REPLACE INTO computations (key, ciphertext) VALUES (?1, ?2)",
            rusqlite::params![key_bytes.to_vec(), serialized_ciphertext],
        )?;
        tx.commit()?;
        Ok(1)  // Return a Result with a value
    }).await?;
    
    println!("Insert completed. Rows affected: {}", result);
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
        let mut stmt = conn.prepare("SELECT key, ciphertext FROM computations")?;
        let rows = stmt.query_map([], |row| {
            let key: Vec<u8> = row.get(0)?;
            let ciphertext: Vec<u8> = row.get(1)?;
            Ok((key, ciphertext))
        })?;

        let mut count = 0;
        for row in rows {
            let (key, ciphertext) = row?;
            println!("Row {}: key length = {}, ciphertext length = {}", 
                count, key.len(), ciphertext.len());
            println!("Key: {:?}", key);
            count += 1;
        }
        println!("Total rows: {}", count);
        Ok(())
    }).await?;
    Ok(())
}