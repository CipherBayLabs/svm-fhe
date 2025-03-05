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

#[derive(Deserialize)]
struct Request {
    value: u8,
}

async fn handle_post(Json(payload): Json<Request>) -> StatusCode {
    println!("Received value: {}", payload.value);
    StatusCode::OK
}


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/post", post(handle_post));

    // Run server
    println!("Server starting on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


////////////////// Database functions //////////////////

