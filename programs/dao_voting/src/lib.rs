use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{Token, TokenAccount};

declare_id!("9bT97oZXoHPW41MayXtEeX4VoUaqkUE1TJvAQkSffRSH");

#[program]
pub mod dao_voting {
    use super::*;

    pub fn new_polling(
        ctx: Context<NewPolling>,
        proposal_voting: String,
        value: String,
        additional_value: String,
    ) -> Result<()> {
        let election = &mut ctx.accounts.election;
        require!(!election.vote_active, CustomError::VoteActive);

        let balance = ctx.accounts.token_account.balance_of(ctx.accounts.authority.key())?;
        require!(balance > 0, CustomError::InsufficientBalance);

        election.id = Clock::get().unwrap().unix_timestamp as u64; // Unique identifier
        election.proposal_voting = proposal_voting;
        election.value = value;
        election.additional_value = additional_value;
        election.vote_active = true;
        election.time = Clock::get().unwrap().unix_timestamp;
        election.creator = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, vote: bool) -> Result<()> {
        let election = &mut ctx.accounts.election;
        require!(election.vote_active, CustomError::VoteInactive);

        let balance = ctx.accounts.token_account.balance_of(ctx.accounts.authority.key())?;
        require!(balance > 0, CustomError::InsufficientBalance);
        require!(!election.voters.contains(&ctx.accounts.authority.key()), CustomError::AlreadyVoted);

        if vote {
            election.current += balance as i64;
        } else {
            election.current -= balance as i64;
        }
        election.number_of_votes += 1;
        election.voters.push(ctx.accounts.authority.key()); // Add voter to the list
        Ok(())
    }

    pub fn to_sum_up(ctx: Context<ToSumUp>) -> Result<()> {
        let election = &mut ctx.accounts.election;
        require!(election.vote_active, CustomError::VoteInactive);
        require!(Clock::get().unwrap().unix_timestamp > election.time || election.number_of_votes >= election.min_votes, CustomError::VotingTime);

        if election.current > 0 {
            if election.proposal_voting == "newSymbol" {
                ctx.accounts.token_account.change_symbol(election.value.clone())?;
            }
            if election.proposal_voting == "newName" {
                ctx.accounts.token_account.change_name(election.value.clone())?;
            }
        }

        election.vote_active = false;
        election.current = 0;
        election.number_of_votes = 0;
        election.proposal_voting = String::new();
        election.value = String::new();
        election.additional_value = String::new();
        election.voters.clear(); // Clear the list of voters

        Ok(())
    }

    pub fn get_results(ctx: Context<GetResults>) -> Result<(i64, u64)> {
        let election = &ctx.accounts.election;
        Ok((election.current, election.number_of_votes))
    }
}

#[account]
pub struct Election {
    pub id: u64, // Unique identifier for the proposal
    pub proposal_voting: String,
    pub value: String,
    pub additional_value: String,
    pub current: i64, // Modified to i64 to accommodate negative votes
    pub number_of_votes: u64,
    pub vote_active: bool,
    pub time: i64,
    pub min_votes: u64,
    pub count: u64,
    pub creator: Pubkey, // The address of the proposal creator
    pub voters: Vec<Pubkey>, // List of voters
}

#[derive(Accounts)]
pub struct NewPolling<'info> {
    #[account(init, payer = authority, space = 8 + 64 + 256 + 256 + 8 + 8 + 8 + 1 + 32 + 8 + 1024)]
    pub election: Account<'info, Election>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + 32 + 256 + 256 + 8 + 8 + 32)]
    pub token_account: Account<'info, ChangableTokenAccount>, // Initialize the token account
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub election: Account<'info, Election>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, ChangableTokenAccount>,
}

#[derive(Accounts)]
pub struct ToSumUp<'info> {
    #[account(mut)]
    pub election: Account<'info, Election>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, ChangableTokenAccount>,
}

#[derive(Accounts)]
pub struct GetResults<'info> {
    pub election: Account<'info, Election>,
}

#[error_code]
pub enum CustomError {
    #[msg("Vote is already active.")]
    VoteActive,
    #[msg("Vote is not active.")]
    VoteInactive,
    #[msg("Insufficient balance.")]
    InsufficientBalance,
    #[msg("Voting time has not ended or minimum votes not reached.")]
    VotingTime,
    #[msg("User has already voted.")]
    AlreadyVoted,
}

#[account]
pub struct ChangableTokenAccount {
    pub name: String,
    pub symbol: String,
    pub balance: u64,
}

impl ChangableToken for ChangableTokenAccount {
    fn change_symbol(&mut self, symbol: String) -> ProgramResult {
        self.symbol = symbol;
        Ok(())
    }

    fn change_name(&mut self, name: String) -> ProgramResult {
        self.name = name;
        Ok(())
    }

    fn balance_of(&self, _user: Pubkey) -> Result<u64> {
        Ok(self.balance)
    }
}

pub trait ChangableToken {
    fn change_symbol(&mut self, symbol: String) -> ProgramResult;
    fn change_name(&mut self, name: String) -> ProgramResult;
    fn balance_of(&self, user: Pubkey) -> Result<u64>;
}
