use axum::http::StatusCode;
use tfhe::{FheUint64, CompressedCiphertextList};
use tfhe::prelude::*;
const DB_PATH: &str = "data/tfhe.db";
use tokio_rusqlite::Connection;


pub async fn get_prepared_ciphertext(key: [u8; 32]) -> Result<FheUint64, StatusCode> {
    let serialized_data = get_ciphertext(key)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let deserialized_compressed: CompressedCiphertextList = bincode::deserialize(&serialized_data)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    deserialized_compressed.get(0)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
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

