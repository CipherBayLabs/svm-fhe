use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

pub fn generate_hash(clock: &Clock) -> [u8; 32] {
    let timestamp = clock.unix_timestamp; 
    let slot = clock.slot;
    
    let mut value = [0u8; 32];
    for i in 0..32 {
        let mixed = (
            (slot.wrapping_mul(1337 + i as u64)) ^
            (timestamp as u64).wrapping_mul(7919 + i as u64)
        ) as u8;
        value[i] = mixed;
    }
    value
}

