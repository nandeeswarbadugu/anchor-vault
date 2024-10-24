use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("2Uw5AaZZaD9dJ1en4qiEnSVAjbjp4cUxUM7aL2w2hNS4");

#[program]
pub mod anchor_vault_q424 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    
}
