use std::io::Cursor;
use std::error::Error;
use tfhe::{
    ConfigBuilder, 
    generate_keys, 
    ServerKey, 
    CompactPublicKey,
    CompactCiphertextList,
    FheUint64,
    set_server_key
};
use tfhe::prelude::{CiphertextList, FheDecrypt};

fn main() -> Result<(), Box<dyn Error>> {
    let config = ConfigBuilder::default()
        .use_custom_parameters(tfhe::shortint::parameters::V0_11_PARAM_MESSAGE_2_CARRY_2_COMPACT_PK_KS_PBS_GAUSSIAN_2M64,)
        .build();
    let (client_key, _server_key) = generate_keys(config);

    let public_key = CompactPublicKey::new(&client_key);
    let compact_lista = CompactCiphertextList::builder(&public_key)
        .push(25u64)
        .build();
    let compact_listb = CompactCiphertextList::builder(&public_key)
        .push(7u64)
        .build();
    
    let expandeda = compact_lista.expand()?;
    let expandedb = compact_listb.expand()?;
    let a: FheUint64 = expandeda.get(0)?.unwrap();
    let b: FheUint64 = expandedb.get(0)?.unwrap();

    println!("Serializing...");
    let mut serialized_data: Vec<u8> = vec![];
    bincode::serialize_into(&mut serialized_data, &_server_key)?;
    bincode::serialize_into(&mut serialized_data, &a)?;
    bincode::serialize_into(&mut serialized_data, &b)?;

    let mut serialized_data = Cursor::new(serialized_data);
    let server_key: ServerKey = bincode::deserialize_from(&mut serialized_data)?;
    let ct_1: FheUint64 = bincode::deserialize_from(&mut serialized_data)?;
    let ct_2: FheUint64 = bincode::deserialize_from(&mut serialized_data)?;

    set_server_key(server_key);
    let result = ct_1 + ct_2;
    let serialized_result = bincode::serialize(&result)?;
    let result: FheUint64 = bincode::deserialize(&serialized_result)?;
    let output: u64 = result.decrypt(&client_key);
    println!("Result: {}", output);
    assert_eq!(output, 32u64);

    Ok(())
}