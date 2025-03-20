use std::io::Cursor;
use std::error::Error;
use tfhe::{
    ConfigBuilder, 
    generate_keys, 
    ServerKey, 
    CompactPublicKey,
    CompressedCiphertextList,
    CompressedCiphertextListBuilder,
    FheUint64,
    FheUint8,
    set_server_key,
};
use tfhe::prelude::{CiphertextList, FheDecrypt, FheEncrypt};
use tfhe::shortint::prelude::PARAM_MESSAGE_2_CARRY_2;
use tfhe::shortint::parameters::COMP_PARAM_MESSAGE_2_CARRY_2;

fn main() -> Result<(), Box<dyn Error>> {

    println!("Generating keys with compression parameters...");
    let config = tfhe::ConfigBuilder::with_custom_parameters(PARAM_MESSAGE_2_CARRY_2)
            .enable_compression(COMP_PARAM_MESSAGE_2_CARRY_2)
            .build();

    let client_key = tfhe::ClientKey::generate(config);
    let sk = tfhe::ServerKey::new(&client_key);

    set_server_key(sk);

    let sender_value = FheUint64::encrypt(100u64, &client_key);

    println!("Encrypted value type: {:?}", std::any::type_name_of_val(&sender_value));

    let compressed = CompressedCiphertextListBuilder::new()
        .push(sender_value)
        .build()
        .map_err(|_| Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Build failed")))?;

    println!("Serializing compressed value...");
    let serialized_data = bincode::serialize(&compressed)
        .map_err(|_| Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Serialize failed")))?;

    println!("Deserializing...");
    let deserialized_compressed: CompressedCiphertextList = bincode::deserialize(&serialized_data)?;

    let value: FheUint64 = deserialized_compressed.get(0)?.unwrap();

    let result = value + FheUint64::encrypt(5u64, &client_key);

    let compressed_result = CompressedCiphertextListBuilder::new()
        .push(result.clone())
        .build()
        .map_err(|_| Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Compress failed")))?;

    let final_serialized = bincode::serialize(&compressed_result)?;

    let output: u64 = result.decrypt(&client_key);
    println!("Final result: {}", output);
    assert_eq!(output, 105u64);

    Ok(())
}