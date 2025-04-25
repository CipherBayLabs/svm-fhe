use anchor_lang::prelude::*;
use fhe_lib::cpi::*;
use fhe_lib::program::FheLib;
use fhe_lib::cpi::accounts::CreateStorage;

declare_id!("AaYfvcZY1iUVFM33KAKUNh8g4JPsStcgp88admDTTMVH");

#[program]
pub mod app {
    use super::*;

    pub fn test_first_add(ctx: Context<UseFhe>, a: [u8;32], b: [u8;32]) -> Result<()> {
        let cpi_accounts = CreateStorage {
            storage: ctx.accounts.storage.to_account_info(),
            signer: ctx.accounts.signer.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_program = CpiContext::new(
            ctx.accounts.fhe_lib.to_account_info(),
            cpi_accounts,
        );
        let ciphertext = fhe_lib::cpi::as_fhe8(cpi_program, a)?;
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
    pub storage: UncheckedAccount<'info>,
}