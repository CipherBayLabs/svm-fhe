use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;
mod utils;
use crate::utils::fhe_types::*;
pub use crate::utils::fhe_types::CipherText;
use crate::utils::internals::generate_hash;
use crate::utils::events::*;

declare_id!("Fuj5qpvT66C7pz4fvyLDV6d8YCUS9idJH2i66Qj5vedh");

#[program]
pub mod fhe_lib {
    use super::*;

    pub fn as_fhe8(ctx: Context<CreateStorage>, key: [u8; 32]) -> Result<CipherText> {
        let storage = &mut ctx.accounts.storage;
        storage.key = key;
        storage.owner = ctx.accounts.signer.key();
        storage.bit_length = 8;
        Ok(CipherText {
            key: storage.key,
            owner: storage.owner,
            bit_length: storage.bit_length,
        })
    }

    pub fn fhe_add(ctx: Context<FheOp>, lhs: CipherText, rhs: CipherText) -> Result<CipherText> {
        let sum = generate_hash(&Clock::get()?);
        emit!(Add8 {
            lhs: lhs.key,
            rhs: rhs.key,
            sum: sum,
        });
        let result = &mut ctx.accounts.result;
        result.key = sum;
        result.owner = ctx.accounts.signer.key();
        result.bit_length = lhs.bit_length;
        Ok(CipherText {
            key: sum,
            owner: ctx.accounts.signer.key(),
            bit_length: lhs.bit_length,
        })
    }

    

}


