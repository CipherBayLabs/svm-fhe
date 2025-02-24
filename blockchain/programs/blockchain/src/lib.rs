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

        // Generate random 32 bytes using the slot number as entropy
        let clock = Clock::get()?;
        let mut value = [0u8; 32];
        value[0..8].copy_from_slice(&clock.slot.to_le_bytes());
        // Fill rest with some deterministic but varying data
        for i in 8..32 {
            value[i] = ((clock.slot + i as u64) % 256) as u8;
        }

        // Store deposit info with random value
        ctx.accounts.deposit_info.owner = ctx.accounts.user.key();
        ctx.accounts.deposit_info.value = value;
        
        msg!("User {} deposited {} lamports", ctx.accounts.user.key(), amount);
        msg!("Deposit info: {:?}", ctx.accounts.deposit_info.value);
        msg!("Deposit info (hex): {:x?}", ctx.accounts.deposit_info.value);
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: [u8; 32], recipient: Pubkey) -> Result<()> {
        // TODO: create helper to check if recipeient already has mapping
        msg!("Transferring {:?} from {:?} to {:?}", amount, ctx.accounts.user.key(), recipient);
        Ok(())
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
        space = 8 + 32 + 32,  // Updated space: discriminator + pubkey + bytes32
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
    pub deposit_info: Account<'info, DepositInfo>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EmitBytes {}