use tfhe::prelude::*;
use tfhe::{ set_server_key, CompactCiphertextList, CompactPublicKey, FheUint64, ServerKey, CompressedCiphertextListBuilder, ClientKey};
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
    server_key: Arc<ServerKey>,
    client_key: Arc<ClientKey>,
}

#[async_trait]
trait KeyAccess {
    fn get_server_key(&self) -> Arc<ServerKey>;
    fn get_client_key(&self) -> Arc<ClientKey>;
}

impl KeyAccess for AppState {
    fn get_server_key(&self) -> Arc<ServerKey> {
        self.server_key.clone()
    }
    fn get_client_key(&self) -> Arc<ClientKey> {
        self.client_key.clone()
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
        server_key: Arc::new(keys::load_server_key()?),
        client_key: Arc::new(keys::load_client_key()?),
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
    let client_key = state.get_client_key();
    let server_key = state.get_server_key();
    set_server_key((*server_key).clone());
    let value = FheUint64::encrypt(payload.value, &*client_key);
    println!("Encrypted value type: {:?}", std::any::type_name_of_val(&value));
    let compressed = CompressedCiphertextListBuilder::new()
        .push(value)
        .build()
        .map_err(|e| {
            println!("Compression error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
    });
    println!("Serializing compressed value...");
    let serialized_data = bincode::serialize(&compressed.unwrap())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    insert_ciphertext(payload.key, serialized_data)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    println!("hit the end of post");
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

    let client_key = keys::load_client_key().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let sender_plain: u64 = sender_value.decrypt(&client_key);
    let recipient_plain: u64 = recipient_value.decrypt(&client_key);
    println!("Sender value: {}, recipient value: {}", sender_plain, recipient_plain);
    //println!("transfer value: {:?}", transfer_value);

    //let condition = sender_value.ge(&transfer_value);
    //let real_amount = condition.if_then_else(&transfer_value, &zero_value);
    println!("about to start operations");
    let new_sender_value = &sender_value - &transfer_value;
    let new_recipient_value = &recipient_value + &transfer_value;
    println!("ending operations");

    // let new_sender_plain: u64 = new_sender_value.decrypt(&client_key);
    // let new_recipient_plain: u64 = new_recipient_value.decrypt(&client_key);
    // println!("New sender value: {}, new recipient value: {}", new_sender_plain, new_recipient_plain);

    let compressed_sender = CompressedCiphertextListBuilder::new()
        .push(new_sender_value.clone())
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    let serialized_sender = bincode::serialize(&compressed_sender)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    update_ciphertext(payload.sender_key, serialized_sender.clone()).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let compressed_recipient = CompressedCiphertextListBuilder::new()
        .push(new_recipient_value.clone())
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    let serialized_recipient = bincode::serialize(&compressed_recipient)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    update_ciphertext(payload.recipient_key, serialized_recipient.clone()).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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
    println!("inserting ciphertext via helper");
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
