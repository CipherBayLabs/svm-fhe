use std::fs;
use std::path::Path;
use tfhe::{ConfigBuilder, generate_keys, ClientKey, ServerKey, CompactPublicKey};

const KEYS_DIR: &str = "keys";
const CLIENT_KEY_PATH: &str = "keys/client_key.bin";
const SERVER_KEY_PATH: &str = "keys/server_key.bin";
const PUBLIC_KEY_PATH: &str = "keys/public_key.bin";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("checking keys...");
    if !Path::new(KEYS_DIR).exists() {
        fs::create_dir(KEYS_DIR)?;
    }

    if Path::new(CLIENT_KEY_PATH).exists() && 
       Path::new(SERVER_KEY_PATH).exists() && 
       Path::new(PUBLIC_KEY_PATH).exists() {
        println!("Keys already exist. Skipping key generation.");
        Ok(())
    } else {
        println!("Generating new keys...");
        let config = ConfigBuilder::default().use_custom_parameters(tfhe::shortint::parameters::V0_11_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64,).build();
        let (client_key, _server_key) = generate_keys(config);
        let public_key = CompactPublicKey::new(&client_key);        
        save_client_key(&client_key)?;
        save_server_key(&_server_key)?;
        save_public_key(&public_key)?;
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

fn save_public_key(key: &CompactPublicKey) -> Result<(), String> {
    let buffer = bincode::serialize(key)
        .map_err(|e| format!("Failed to serialize public key: {}", e))?;
    fs::write(PUBLIC_KEY_PATH, buffer)
        .map_err(|e| format!("Failed to save public key: {}", e))?;
    Ok(())
}

pub fn load_public_key() -> Result<CompactPublicKey, String> {
    let data = fs::read(PUBLIC_KEY_PATH).map_err(|e| e.to_string())?;
    bincode::deserialize(&data).map_err(|e| e.to_string())
}