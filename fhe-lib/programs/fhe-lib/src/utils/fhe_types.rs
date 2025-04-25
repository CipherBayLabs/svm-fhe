use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CipherText {
    pub key: [u8; 32],
    pub owner: Pubkey,
    pub bit_length: u16,
}

#[derive(Accounts)]
#[instruction(key: [u8; 32])]
pub struct CreateStorage<'info>{
    #[account(
        init,
        payer = signer,
        space = 8 + CipherText::INIT_SPACE,
        seeds = [b"fhe_storage", key.as_ref()],
        bump
    )]
    pub storage: Account<'info, CipherText>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(key: [u8; 32])]
pub struct FheOp<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    
    #[account(
        init,
        payer = signer,
        space = 8 + CipherText::INIT_SPACE,
        seeds = [b"fhe_storage", key.as_ref()],
        bump
    )]
    pub result: Account<'info, CipherText>,
}


