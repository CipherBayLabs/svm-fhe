use anchor_lang::prelude::*;
use fhe_lib::cpi::*;
use fhe_lib::program::FheLib;
use fhe_lib::cpi::accounts::CreateStorage;
use fhe_lib::cpi::accounts::FheOp;
use fhe_lib::CipherText;

declare_id!("AaYfvcZY1iUVFM33KAKUNh8g4JPsStcgp88admDTTMVH");

#[program]
pub mod app {
    use super::*;

    pub fn test_first_add(ctx: Context<UseFhe>, a: [u8;32], b: [u8;32]) -> Result<()> {

        let cpi_accounts_a = CreateStorage {
            storage: ctx.accounts.storage_a.to_account_info(),
            signer: ctx.accounts.signer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_program_a = CpiContext::new(
            ctx.accounts.fhe_lib.to_account_info(),
            cpi_accounts_a,
        );
        let ciphertext_a = fhe_lib::cpi::as_fhe8(cpi_program_a, a)?;

        let cpi_accounts_b = CreateStorage {
            storage: ctx.accounts.storage_b.to_account_info(),
            signer: ctx.accounts.signer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_program_b = CpiContext::new(
            ctx.accounts.fhe_lib.to_account_info(),
            cpi_accounts_b,
        );
        let ciphertext_b = fhe_lib::cpi::as_fhe8(cpi_program_b, b)?;
      
        // Manually create CipherText structs from account data
        let ciphertext_a = CipherText {
            key: a,  // Use the original key
            owner: ctx.accounts.signer.key(),
            bit_length: 8,
        };
        
        let ciphertext_b = CipherText {
            key: b,  // Use the original key
            owner: ctx.accounts.signer.key(),
            bit_length: 8,
        };

        //add the two ciphertexts
        let cpi_accounts_sum = FheOp {
            signer: ctx.accounts.signer.to_account_info(),
            result: ctx.accounts.storage_sum.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };

        let cpi_program_sum = CpiContext::new(
            ctx.accounts.fhe_lib.to_account_info(),
            cpi_accounts_sum,
        );
        let sum_ciphertext = fhe_lib::cpi::fhe_add(cpi_program_sum, ciphertext_a, ciphertext_b)?;

        Ok(())

    }
}

#[derive(Accounts)]
pub struct UseFhe<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub fhe_lib: Program<'info, FheLib>,
    /// CHECK: This account is initialized by the fhe_lib program in the CPI call
    #[account(mut)]
    pub storage_a: UncheckedAccount<'info>,
    /// CHECK: This account is initialized by the fhe_lib program in the CPI call
    #[account(mut)]
    pub storage_b: UncheckedAccount<'info>,
    /// CHECK: This account stores the result of the addition
    #[account(mut)]
    pub storage_sum: UncheckedAccount<'info>,
}

