use anchor_lang::prelude::*;

declare_id!("2ZKcfLiwNShCekPnsgq1hQSmGF1fU6iT4Q3rrMXet4p2");

#[program]
pub mod dao_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
