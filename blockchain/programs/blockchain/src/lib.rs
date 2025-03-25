use anchor_lang::prelude::*;

declare_id!("GEFoAn6CNJiG9dq8xgm24fjzjip7n5GcH5AyqVC6QzdD");

#[program]
pub mod blockchain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, amount)?;

        // Get slot and recent blockhash for entropy
        let clock = Clock::get()?;
        let mut value = [0u8; 32];
        
        // Use more sources of entropy
        let timestamp = clock.unix_timestamp;
        let slot = clock.slot;
        
        // Mix values throughout the array
        for i in 0..32 {
            let mixed = (
                (slot.wrapping_mul(1337 + i as u64)) ^
                (timestamp as u64).wrapping_mul(7919 + i as u64)
            ) as u8;
            value[i] = mixed;
        }
        
        ctx.accounts.deposit_info.owner = ctx.accounts.user.key();
        ctx.accounts.deposit_info.value = value;
        
        msg!("User {} deposited {} lamports", ctx.accounts.user.key(), amount);
        msg!("Deposit info: {:?}", ctx.accounts.deposit_info.value);
        msg!("Deposit info (hex): {:x?}", ctx.accounts.deposit_info.value);
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: [u8; 32], recipient: Pubkey) -> Result<()> {
        // Emit both sender's and recipient's ciphertext values
        msg!("Sender's deposit value: {:?}", ctx.accounts.sender_deposit.value);
        msg!("Recipient's deposit value: {:?}", ctx.accounts.recipient_deposit.value);
        msg!("Transferring {:?} from {:?} to {:?}", amount, ctx.accounts.user.key(), ctx.accounts.recipient.key());
        Ok(())
    }

    pub fn view_balance(ctx: Context<ViewBalance>) -> Result<[u8; 32]> {
        // Simply return the stored value bytes
        Ok(ctx.accounts.deposit_info.value)
    }

    pub fn emit_bytes(ctx: Context<EmitBytes>, value: [u8; 32]) -> Result<()> {
        msg!("Emitting bytes: {:?}", value);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct DepositInfo {
    owner: Pubkey,    // 32 bytes
    value: [u8; 32],  // 32 bytes instead of u64
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 32, 
        seeds = [user.key().as_ref()],
        bump
    )]
    pub deposit_info: Account<'info, DepositInfo>,

    /// CHECK: This is the PDA that will hold SOL
    #[account(
        mut,
        seeds = [b"vault"],
        bump
    )]
    pub vault: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(value: [u8; 32])]
pub struct Transfer<'info> {
    #[account(
        mut,
        seeds = [user.key().as_ref()],
        bump
    )]
    pub sender_deposit: Account<'info, DepositInfo>,

    #[account(
        mut,
        seeds = [recipient.key().as_ref()],
        bump
    )]
    pub recipient_deposit: Account<'info, DepositInfo>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: This is just for logging the recipient's address
    pub recipient: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EmitBytes {}

#[derive(Accounts)]
pub struct ViewBalance<'info> {
    #[account(
        seeds = [user.key().as_ref()],
        bump,
    )]
    pub deposit_info: Account<'info, DepositInfo>,
    pub user: Signer<'info>,
}