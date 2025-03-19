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

fn main() -> Result<(), Box<dyn Error>> {
    let config = ConfigBuilder::default()
        .use_custom_parameters(tfhe::shortint::parameters::V0_11_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64,)
        .build();
    let (client_key, server_key) = generate_keys(config);

    let public_key = CompactPublicKey::new(&client_key);

    let sender_value = FheUint64::encrypt(100u64, &client_key);

    let compressed = CompressedCiphertextListBuilder::new()
        .push(sender_value)
        .build()
        .map_err(|_| Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Build failed")))?;

    println!("Serializing compressed value...");
    let serialized_data = bincode::serialize(&compressed.unwrap())
        .map_err(|_| Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Serialize failed")))?;

    println!("Deserializing...");
    let deserialized_compressed: CompressedCiphertextList = bincode::deserialize(&serialized_data)?;

    let expanded = deserialized_compressed.expand()?;
    let value: FheUint64 = expanded.get(0)?.unwrap();

    set_server_key(server_key);
    let result = value + FheUint64::encrypt(5u64, &client_key);

    let compressed_result = CompressedCiphertextListBuilder::new()
        .push(&result)
        .build();

    let final_serialized = bincode::serialize(&compressed_result)?;

    let output: u64 = result.decrypt(&client_key);
    println!("Final result: {}", output);
    assert_eq!(output, 105u64);

    Ok(())
}