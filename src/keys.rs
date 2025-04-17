use std::fs;
use std::path::Path;
use tfhe::{ConfigBuilder, generate_keys, ClientKey, ServerKey, CompactPublicKey};
use tfhe::shortint::prelude::PARAM_MESSAGE_2_CARRY_2;
use tfhe::shortint::parameters::COMP_PARAM_MESSAGE_2_CARRY_2;

const KEYS_DIR: &str = "keys";
const CLIENT_KEY_PATH: &str = "keys/client_key.bin";
const SERVER_KEY_PATH: &str = "keys/server_key.bin";
//const PUBLIC_KEY_PATH: &str = "keys/public_key.bin";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("checking keys...");
    if !Path::new(KEYS_DIR).exists() {
        fs::create_dir(KEYS_DIR)?;
    }

    if Path::new(CLIENT_KEY_PATH).exists() && 
       Path::new(SERVER_KEY_PATH).exists() {
        println!("Keys already exist. Skipping key generation.");
        Ok(())
    } else {
        println!("Generating new keys...");
        let config = tfhe::ConfigBuilder::with_custom_parameters(PARAM_MESSAGE_2_CARRY_2)
        .enable_compression(COMP_PARAM_MESSAGE_2_CARRY_2).build();
        let client_key = tfhe::ClientKey::generate(config);
        let server_key = tfhe::ServerKey::new(&client_key);
        save_client_key(&client_key)?;
        save_server_key(&server_key)?;
        println!("Keys generated successfully.");
        Ok(())
    }
}

fn save_client_key(key: &ClientKey) -> Result<(), String> {
    let buffer = bincode::serialize(key)
        .map_err(|e| format!("Failed to serialize client key: {}", e))?;
    fs::write(CLIENT_KEY_PATH, buffer)
        .map_err(|e| format!("Failed to save client key: {}", e))?;
    Ok(())
}

fn save_server_key(key: &ServerKey) -> Result<(), String> {
    let buffer = bincode::serialize(key)
        .map_err(|e| format!("Failed to serialize server key: {}", e))?;
    fs::write(SERVER_KEY_PATH, buffer)
        .map_err(|e| format!("Failed to save server key: {}", e))?;
    Ok(())
}

// fn save_public_key(key: &CompactPublicKey) -> Result<(), String> {
//     let buffer = bincode::serialize(key)
//         .map_err(|e| format!("Failed to serialize public key: {}", e))?;
//     fs::write(PUBLIC_KEY_PATH, buffer)
//         .map_err(|e| format!("Failed to save public key: {}", e))?;
//     Ok(())
// }

// pub fn load_public_key() -> Result<CompactPublicKey, String> {
//     let data = fs::read(PUBLIC_KEY_PATH).map_err(|e| e.to_string())?;
//     bincode::deserialize(&data).map_err(|e| e.to_string())
// }

pub fn load_client_key() -> Result<ClientKey, String> {
    let data = fs::read(CLIENT_KEY_PATH)
        .map_err(|e| format!("Failed to read client key: {}", e))?;
    
    bincode::deserialize(&data)
        .map_err(|e| format!("Failed to deserialize client key: {}", e))
}

pub fn load_server_key() -> Result<ServerKey, String> {
    let data = fs::read(SERVER_KEY_PATH)
        .map_err(|e| format!("Failed to read server key: {}", e))?;
    bincode::deserialize(&data).map_err(|e| e.to_string())
}