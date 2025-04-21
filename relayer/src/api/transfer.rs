use anyhow::Result;
use serde::Serialize;
use tokio::runtime::Handle;

#[derive(Serialize)]
struct TransferRequest {
    sender_key: [u8; 32],
    recipient_key: [u8; 32],
    transfer_value: [u8; 32],
}

#[derive(Serialize)]
struct DepositRequest {
    value: u64,
    key: [u8; 32],
}

pub async fn transfer(sender: [u8; 32], recipient: [u8; 32], amount: [u8; 32]) -> Result<()> {
    let request = TransferRequest {
        sender_key: sender,
        recipient_key: recipient,
        transfer_value: amount,
    };
    println!("Sending transfer request to backend");
    let result = tokio::task::block_in_place(move || {
        let response = ureq::post("http://localhost:3000/transfer")
            .set("Content-Type", "application/json")
            .send_json(&request);
        Ok(())
    });
    result
}

pub async fn deposit(value: u64, key: [u8; 32]) -> Result<()> {
    let request = DepositRequest {
        value,
        key,
    };
    
    let result = tokio::task::block_in_place(move || {  
        // Send the request and check response
        match ureq::post("http://localhost:3000/post")
            .set("Content-Type", "application/json")
            .send_json(&request)
        {
            Ok(response) => {
                println!("Deposit successful: status {}", response.status());
                Ok(())
            },
            Err(err) => {
                println!("Deposit failed: {}", err);
                // Return the error instead of ignoring it
                Err(anyhow::anyhow!("Failed to send deposit: {}", err))
            }
        }
    });
    
    result
}
