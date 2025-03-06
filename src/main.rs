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
mod keys;

const DB_PATH: &str = "data/tfhe.db";

#[derive(Clone)]
struct AppState {
    db: Arc<Connection>,
    pubkey: Arc<CompactPublicKey>,
}

#[async_trait]
trait PubkeyAccess {
    fn get_pubkey(&self) -> Arc<CompactPublicKey>;
}

impl PubkeyAccess for AppState {
    fn get_pubkey(&self) -> Arc<CompactPublicKey> {
        self.pubkey.clone()
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
    value: u64,
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
    };
    let app = Router::new()
        .route("/post", post(handle_post))
        .with_state(state);

    println!("Server starting on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

////////////////// Database endpoint functions //////////////////

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

async fn get_ciphertext(conn: &Connection, key: &[u8; 32]) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT ciphertext FROM computations WHERE key = ?")?;
    let ciphertext = stmt.query_row(key, |row| row.get::<Vec<u8>>("ciphertext"))?;
    Ok(ciphertext)
}