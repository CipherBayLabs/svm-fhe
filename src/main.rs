use tfhe::prelude::*;
use tfhe::{ set_server_key, CompactCiphertextList, CompactPublicKey, FheUint64, ServerKey };
use std::io::Cursor;
use axum::{
    routing::{get, post}, Router, Json, extract::State,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_rusqlite::Connection;
use std::path::Path;
use std::fs;
use async_trait::async_trait;
use tokio::try_join;
mod keys;
mod operations;

const DB_PATH: &str = "data/tfhe.db";
const zero_key: [u8; 32] = [0u8; 32];

#[derive(Clone)]
struct AppState {
    db: Arc<Connection>,
    pubkey: Arc<CompactPublicKey>,
    server_key: Arc<ServerKey>,
}

#[async_trait]
trait PubkeyAccess {
    fn get_pubkey(&self) -> Arc<CompactPublicKey>;
    fn get_server_key(&self) -> Arc<ServerKey>;
}

impl PubkeyAccess for AppState {
    fn get_pubkey(&self) -> Arc<CompactPublicKey> {
        self.pubkey.clone()
    }
    fn get_server_key(&self) -> Arc<ServerKey> {
        self.server_key.clone()
    }
}

////////////////// Request structs //////////////////

#[derive(Deserialize)]
struct Request {
    key: [u8; 32],
    value: u64,
}

#[derive(Deserialize)]
struct Transfer {
    sender_key: [u8; 32],
    recipient_key: [u8; 32],
    transfer_value: [u8; 32],
}

////////////////// Main function //////////////////

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new("data").exists() {
        fs::create_dir("data").expect("Failed to create data directory");
    }
    let state = AppState {
        db: Arc::new(Connection::open(DB_PATH).await?),
        pubkey: Arc::new(keys::load_public_key()?),
        server_key: Arc::new(keys::load_server_key()?),
    };
    init_db(&state.db).await?;
    let app = Router::new()
        .route("/post", post(handle_post))
        .route("/transfer", post(handle_transfer))
        .with_state(state);

    println!("Server starting on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

////////////////// Server endpoint functions //////////////////

async fn handle_post(State(state): State<AppState>, Json(payload): Json<Request>) -> Result<StatusCode, StatusCode> {
    println!("Received value: {}, key: {:?}", payload.value, payload.key);
    let public_key = state.get_pubkey();
    let compact_list = CompactCiphertextList::builder(&public_key)
        .push(payload.value)
        .build();
    let serialized_list = bincode::serialize(&compact_list)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let conn = Connection::open(DB_PATH)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    conn.call(move |conn| {
        conn.execute(
            "INSERT OR REPLACE INTO computations (key, ciphertext) VALUES (?1, ?2)",
            (payload.key, serialized_list),
        ).map_err(|e| {
            println!("Insert error: {}", e);
            e
        })?;
        Ok(())
    }).await;
    println!("Successfully saved to database!");
    Ok(StatusCode::OK)
}

async fn handle_transfer(State(state): State<AppState>, Json(payload): Json<Transfer>) -> Result<StatusCode, StatusCode> {
    println!("=== TRANSFER REQUEST RECEIVED ===");
    let server_key = state.get_server_key();
    set_server_key((*server_key).clone());
    println!("handle_transfer hit!!!!!!!!");

    println!("Attempting to fetch sender ciphertext...");
    println!("Sender key: {:?}", payload.sender_key);
    println!("Reciver key: {:?}", payload.recipient_key);
    println!("transfer key: {:?}", payload.transfer_value);
    let sender_value = operations::get_prepared_ciphertext(payload.sender_key)
        .await
        .map_err(|e| {
            println!("Error fetching sender value: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    println!("Successfully got sender value");

    println!("Attempting to fetch recipient ciphertext...");
    let recipient_value = operations::get_prepared_ciphertext(payload.recipient_key)
        .await
        .map_err(|e| {
            println!("Error fetching recipient value: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    println!("Successfully got recipient value");

    println!("Attempting to fetch transfer value...");
    
    let transfer_value = operations::get_prepared_ciphertext(payload.transfer_value)
        .await
        .map_err(|e| {
            println!("Error fetching transfer value: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    println!("Successfully got transfer value");

    println!("Attempting to fetch zero value...");
    let zero_value = operations::get_prepared_ciphertext(zero_key)
        .await
        .map_err(|e| {
            println!("Error fetching zero value: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    println!("Successfully got zero value");

    // let client_key = keys::load_client_key().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // let sender_plain: u64 = sender_value.decrypt(&client_key);
    // let recipient_plain: u64 = recipient_value.decrypt(&client_key);
    // println!("Sender value: {}, recipient value: {}", sender_plain, recipient_plain);
    // println!("transfer value: {:?}", transfer_value);

    //let condition = sender_value.ge(&transfer_value);
    //let real_amount = condition.if_then_else(&transfer_value, &zero_value);
    let new_sender_value = &sender_value - &transfer_value;
    let new_recipient_value = &recipient_value + &transfer_value;

    // let new_sender_plain: u64 = new_sender_value.decrypt(&client_key);
    // let new_recipient_plain: u64 = new_recipient_value.decrypt(&client_key);
    // println!("New sender value: {}, new recipient value: {}", new_sender_plain, new_recipient_plain);
    let serialized_sender = bincode::serialize(&new_sender_value).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    update_ciphertext(payload.sender_key, serialized_sender).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // let serialized_recipient = bincode::serialize(&new_recipient_value).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // update_ciphertext(payload.recipient_key, serialized_recipient).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // let client_key = keys::load_client_key().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // let sender_value: u64 = sender_value.decrypt(&client_key);
    // let recipient_value: &u64 = &recipient_value.decrypt(&client_key);
    // println!("Sender value: {}, recipient value: {}", sender_value, recipient_value);
    Ok(StatusCode::OK)
}

////////////////// Database helper functions //////////////////

async fn init_db(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = Path::new(DB_PATH).parent() {
        println!("Creating directory at: {:?}", parent);
        fs::create_dir_all(parent)?;
    }
    conn.call(|conn| {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS computations (
                key CHAR(32) NOT NULL PRIMARY KEY,
                ciphertext BLOB NOT NULL
            )",
            (),
        ).map_err(|e| {
            println!("Database error: {}", e);
            e
        })?;
        Ok(())
    })
    .await;
    Ok(())
}

pub async fn update_ciphertext(key: [u8; 32], new_ciphertext: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("data/tfhe.db").await?;
    
    conn.call(move |conn| {
        // First find the row with the matching key
        let mut stmt = conn.prepare(
            "UPDATE computations SET ciphertext = ? WHERE key = ?"
        )?;
        let rows_affected = stmt.execute((&new_ciphertext, &key))?;
        if rows_affected == 0 {
            println!("No row found with the given key");
        } else {
            println!("Updated ciphertext for key: {:?}", key);
        }
        Ok(())
    }).await?;

    Ok(())
}

pub async fn get_ciphertext(key: [u8; 32]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let conn = Connection::open(DB_PATH).await?;
    
    conn.call(move |conn| {
        conn.query_row(
            "SELECT ciphertext FROM computations WHERE key = ?",
            [key],
            |row| row.get(0)
        )
    }).await.map_err(Into::into)
}

pub async fn insert_ciphertext(key: [u8; 32], ciphertext: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(DB_PATH).await?;
    conn.call(move |conn| {
        conn.execute(
            "INSERT OR REPLACE INTO computations (key, ciphertext) VALUES (?1, ?2)",
            (key, ciphertext),
        ).map_err(|e| {
            println!("Insert error: {}", e);
            e
        })?;
        Ok(())
    }).await?;
    Ok(())
}
