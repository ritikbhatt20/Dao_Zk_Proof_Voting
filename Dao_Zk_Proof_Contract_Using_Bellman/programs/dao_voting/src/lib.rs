use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{self, TokenAccount};
use solana_program::account_info::AccountInfo;
use solana_program::program_pack::Pack;
use spl_token::state::Account as SplTokenAccount;
use bellman::groth16::{Proof, prepare_verifying_key, verify_proof};
use bls12_381::{Bls12, Scalar};
use std::str::FromStr;
use bellman::groth16::VerifyingKey as GrothVerifyingKey;
use anchor_lang::error::AnchorError;

mod constants;
mod state;
mod errors;

use crate::{constants::*, state::*, errors::*};

declare_id!("3XuNmJEHjuk5Vo7U6fAPp1vJekyW2GJsSmWBWkLjnbyK");

#[program]
pub mod dao_voting {
    use std::num::NonZeroI128;

    use super::*;

    pub fn new_polling(
        ctx: Context<NewPolling>,
        token: Pubkey,
        proposal_voting: String,
        value: String,
        additional_value: String,
    ) -> Result<()> {
        let election = &mut ctx.accounts.election;

        require!(!election.vote_active, CustomError::VoteActive);

        let balance = get_token_balance(&ctx.accounts.token_account.to_account_info())?;
        require!(balance > 0, CustomError::InsufficientBalance);

        let now = Clock::get().unwrap().unix_timestamp;
        election.id = now as u64; // Unique identifier
        election.token = token;
        election.proposal_voting = proposal_voting;
        election.value = value;
        election.additional_value = additional_value;
        election.vote_active = true;
        election.time = now;
        election.creator = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn vote(
        ctx: Context<Vote>, 
        vote: bool, 
        zk_proof: Vec<u8>, 
        public_input: String
    ) -> Result<()> {
        let election = &mut ctx.accounts.election;
        require!(election.vote_active, CustomError::VoteInactive);
    
        let balance = get_token_balance(&ctx.accounts.token_account.to_account_info())?;
        require!(balance > 0, CustomError::InsufficientBalance);
        require!(!election.voters.contains(&ctx.accounts.authority.key()), CustomError::AlreadyVoted);
    
        verify_zk_proof(zk_proof, public_input, &ctx.accounts.verifying_key.key)?;

        if vote {
            election.current += balance as i64;
        } else {
            election.current -= balance as i64;
        }
        election.number_of_votes += 1;
        election.voters.push(ctx.accounts.authority.key()); // Add voter to the list

        // Create or update user account with reward points
        let user = &mut ctx.accounts.user;
        user.pubkey = ctx.accounts.authority.key();
        user.reward_points += 1; // Award 1 reward point for voting

        Ok(())
    }

    pub fn to_sum_up(ctx: Context<ToSumUp>) -> Result<()> {
        let election = &mut ctx.accounts.election;
        require!(election.vote_active, CustomError::VoteInactive);
        require!(Clock::get().unwrap().unix_timestamp > election.time || election.number_of_votes >= election.min_votes, CustomError::VotingTime);

        if election.current > 0 {
            if election.proposal_voting == "newSymbol" {
                ctx.accounts.changable_token_account.change_symbol(election.value.clone())?;
            }
            if election.proposal_voting == "newName" {
                ctx.accounts.changable_token_account.change_name(election.value.clone())?;
            }
        }

        Ok(())
    }

    pub fn get_results(ctx: Context<GetResults>) -> Result<()> {
        let election = &mut ctx.accounts.election;

        msg!("Total number of votes : {}", election.number_of_votes);
        msg!("Current Votes Balance : {}", election.current);

        if election.current > 0 {
            msg!("Proposal Passed");
        } else {
            msg!("Proposal Rejected");
        }

        election.reset();
        Ok(())
    }

    pub fn close_election(ctx: Context<CloseElection>) -> Result<()> {
        let election = &mut ctx.accounts.election;
        require!(!election.vote_active, CustomError::VoteActive);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct NewPolling<'info> {
    #[account(
        init,
        payer = authority, 
        space = 8 + 64 + 256 + 256 + 8 + 8 + 8 + 1 + 32 + 8 + 1024,
        seeds = [ELECTION_SEED.as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub election: Account<'info, Election>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(
        init, 
        payer = authority, 
        space = 8 + 32 + 256 + 256 + 8 + 8 + 32,
        seeds = [CHANGABLE_TOKEN_SEED.as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub changable_token_account: Account<'info, ChangableTokenAccount>, // Initialize the token account
    pub verifying_key: Account<'info, VerifyingKey>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(
        mut,
        seeds = [ELECTION_SEED.as_bytes(), election.creator.as_ref()],
        bump,
    )]
    pub election: Account<'info, Election>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [CHANGABLE_TOKEN_SEED.as_bytes(), election.creator.as_ref()],
        bump,
    )]
    pub changable_token_account: Account<'info, ChangableTokenAccount>,
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + 32 + 8,
        seeds = [USER_SEED.as_bytes(), authority.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,
    pub verifying_key: Account<'info, VerifyingKey>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ToSumUp<'info> {
    #[account(
        mut,
        seeds = [ELECTION_SEED.as_bytes(), election.creator.as_ref()],
        bump,
    )]
    pub election: Account<'info, Election>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [CHANGABLE_TOKEN_SEED.as_bytes(), election.creator.as_ref()],
        bump,
    )]
    pub changable_token_account: Account<'info, ChangableTokenAccount>,
}

#[derive(Accounts)]
pub struct GetResults<'info> {
    #[account(
        mut,
        seeds = [ELECTION_SEED.as_bytes(), election.creator.as_ref()],
        bump,
    )]
    pub election: Account<'info, Election>,
}

#[derive(Accounts)]
pub struct CloseElection<'info> {
    #[account(
        mut,
        close = authority,
        seeds = [ELECTION_SEED.as_bytes(), election.creator.as_ref()],
        bump
    )]
    pub election: Account<'info, Election>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        close = authority,
        seeds = [CHANGABLE_TOKEN_SEED.as_bytes(), election.creator.as_ref()],
        bump
    )]
    pub changable_token_account: Account<'info, ChangableTokenAccount>,
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

#[account]
pub struct VerifyingKey {
    pub key: Vec<u8>,
}

fn get_token_balance(account: &AccountInfo) -> Result<u64> {
    let data = &account.try_borrow_data()?;
    let token_account = SplTokenAccount::unpack(data)?;
    Ok(token_account.amount)
}

fn verify_zk_proof(zk_proof: Vec<u8>, public_input: String, verifying_key: &[u8]) -> Result<()> {
    // Deserialize and verify zk-SNARK proof
    let proof = Proof::<Bls12>::read(&zk_proof[..]).map_err(|_| CustomError::ProofDeserializationFailed)?;

    // Convert public input from string to Scalar
    let bytes = hex::decode(&public_input).map_err(|_| CustomError::InvalidPublicInput)?;
    require!(bytes.len() == 32, CustomError::InvalidPublicInput); // Ensure bytes length is correct

    let mut array = [0u8; 64]; // Use a 64-byte array
    array[..32].copy_from_slice(&bytes);
    let scalar = Scalar::from_bytes_wide(&array);

    let vk = GrothVerifyingKey::<Bls12>::read(verifying_key).map_err(|_| CustomError::ProofVerificationFailed)?;
    let pvk = prepare_verifying_key(&vk);
    let result = verify_proof(&pvk, &proof, &[scalar]);

    if result.is_err() {
        return Err(CustomError::ProofVerificationFailed.into());
    }
    Ok(())
}

impl Election {
    pub fn reset(&mut self) {
        self.vote_active = false;
        self.current = 0;
        self.number_of_votes = 0;
        self.proposal_voting.clear();
        self.value.clear();
        self.additional_value.clear();
        self.voters.clear();
    }
}