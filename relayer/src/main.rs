use solana_client::{
    rpc_client::RpcClient,
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcProgramAccountsConfig, RpcTransactionLogsFilter,RpcTransactionLogsConfig},
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::str::FromStr;
use anyhow::Result;
use futures_util::StreamExt; 
use async_trait::async_trait;
mod listener;
use listener::utils::parse_array_from_log;
mod api;
use api::transfer::{transfer, deposit};

struct SolanaConnection {
    client: RpcClient,
    ws_url: String,
    program_id: Pubkey,
}

#[async_trait]
trait InstructionListener {
    fn instruction_prefix(&self) -> &str;
    async fn process_instruction(&self, instruction: &str) -> Result<()>;
}


#[async_trait]
trait ProgramListenerTransfer {
    async fn listen(&self) -> Result<()>;
}

#[async_trait]
trait BackendRelayer {
    async fn relay_event(&self, event: &str) -> Result<()>;
}

impl SolanaConnection {
    pub fn new(rpc_url: &str, ws_url: &str, program_id_str: &str) -> Result<Self> {
        let client = RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        );
        let program_id = Pubkey::from_str(program_id_str)?;
        
        Ok(Self {
            client,
            program_id,
            ws_url: ws_url.to_string(),
        })
    }
}

#[async_trait]
impl ProgramListenerTransfer for SolanaConnection {
    async fn listen(&self) -> Result<()> {
        let pubsub_client = PubsubClient::new(&self.ws_url).await?;

        let logs_config = RpcTransactionLogsConfig {
            commitment: Some(CommitmentConfig::confirmed()),
        };

        let (mut logs_subscription, _) = pubsub_client
        .logs_subscribe(
            RpcTransactionLogsFilter::Mentions(vec![self.program_id.to_string()]),
            logs_config,
        )
        .await?;

        while let Some(response) = logs_subscription.next().await {
            for log in &response.value.logs {
                if log.contains("Instruction: Transfer") {
                    println!("Transfer event detected!");
                    struct TransferData {
                        pattern: &'static str,
                        description: &'static str, 
                        value: Option<[u8; 32]>,
                    }
                    let mut transfer_data = vec![
                        TransferData { pattern: "Sender's deposit value:", description: "Sender's value", value: None },
                        TransferData { pattern: "Recipient's deposit value:", description: "Recipient's value", value: None },
                        TransferData { pattern: "Transferring", description: "Transfer amount", value: None },
                    ];
                    for detail_log in &response.value.logs {
                        for data in &mut transfer_data {
                            if detail_log.contains(data.pattern) {
                                data.value = parse_array_from_log(detail_log);
                                if let Some(arr) = data.value {
                                    println!("{}: {:?}", data.description, arr);
                                }
                            }
                        }
                    }
                    let sender_value = transfer_data[0].value;
                    let recipient_value = transfer_data[1].value;
                    let transfer_amount = transfer_data[2].value;
                    if let (Some(sender), Some(recipient), Some(amount)) = 
                        (sender_value, recipient_value, transfer_amount) {
                        println!("Complete transfer detected:");
                        println!("  From: {:?}", sender);
                        println!("  To:   {:?}", recipient);
                        println!("  Amount: {:?}", amount);
                        transfer(sender, recipient, amount).await?;
                    }
                }

                if log.contains("Instruction: Deposit") {
                    println!("Deposit event detected!");
                    
                    struct DepositData {
                        pattern: &'static str,
                        description: &'static str, 
                        value: Option<[u8; 32]>,
                    }
                    
                    let mut deposit_data = vec![
                        DepositData { pattern: "Deposit info:", description: "Deposit ciphertext", value: None },
                    ];
                    
                    let mut lamport_value: Option<u64> = None;
                    
                    // Process all logs against patterns
                    for detail_log in &response.value.logs {
                        // Extract the ciphertext
                        for data in &mut deposit_data {
                            if detail_log.contains(data.pattern) {
                                data.value = parse_array_from_log(detail_log);
                                if let Some(arr) = data.value {
                                    println!("{}: {:?}", data.description, arr);
                                }
                            }
                        }
                        
                        // Extract the lamport amount 
                        if detail_log.contains("deposited") && detail_log.contains("lamports") {
                            let parts: Vec<&str> = detail_log.split_whitespace().collect();
                            // Format: "... deposited <AMOUNT> lamports"
                            for (i, part) in parts.iter().enumerate() {
                                if part.contains("deposited") && i + 1 < parts.len() {
                                    if let Ok(amount) = parts[i + 1].parse::<u64>() {
                                        lamport_value = Some(amount);
                                    }
                                }
                            }
                        }
                    }
                    
                    let ciphertext = deposit_data[0].value;
                    
                    // Process the complete deposit if we have all needed data
                    if let (Some(amount), Some(cipher)) = (lamport_value, ciphertext) {
                        println!("Complete deposit detected:");
                        println!("  Amount: {} lamports", amount);
                        println!("  Ciphertext: {:?}", cipher);
                        deposit(amount, cipher).await?;
                    }
                }
            }
        }
        Ok(())
    }
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {  
    let connection = SolanaConnection::new(
        "http://localhost:8899",
        "ws://localhost:8900",
        "GEFoAn6CNJiG9dq8xgm24fjzjip7n5GcH5AyqVC6QzdD"
    )?;

    println!("Starting Solana relayer...");
    connection.listen().await?;
    Ok(())
}

