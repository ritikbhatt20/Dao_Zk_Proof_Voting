use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{self, TokenAccount};
use solana_program::account_info::AccountInfo;
use solana_program::program_pack::Pack;
use spl_token::state::Account as SplTokenAccount;
use bellman::groth16::{Proof, prepare_verifying_key, verify_proof};
use std::str::FromStr;
use bellman::groth16::VerifyingKey as GrothVerifyingKey;
use anchor_lang::error::AnchorError;
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;

pub mod zk_proof;
pub mod constants;
pub mod state;
pub mod errors;

use crate::{constants::*, state::*, errors::*, zk_proof::*};

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

        election.id = Clock::get().unwrap().unix_timestamp as u64; // Unique identifier
        election.token = token;
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
    
        let balance = get_token_balance(&ctx.accounts.token_account.to_account_info())?;
        require!(balance > 0, CustomError::InsufficientBalance);
        require!(!election.voters.contains(&ctx.accounts.authority.key()), CustomError::AlreadyVoted);
    
        // Encrypt the vote
        let public_key = ElGamalPubkey { compressed_point: [0; 32] }; // Use the correct public key
        let ciphertext = zk_proof::elgamal_file::perform_encryption(vote, &public_key);
    
        // Generate the vote proof
        let random_scalar = Scalar::random(&mut rand::rngs::OsRng);  // Use the randomness
        let proof = create_vote_proof(vote, random_scalar, &public_key);
    
        // Verify the proof before adding it
        require!(validate_vote_proof(&proof, &ciphertext, &public_key), CustomError::InvalidProof);
    
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

        election.vote_active = false;
        election.current = 0;
        election.number_of_votes = 0;
        election.proposal_voting = String::new();
        election.value = String::new();
        election.additional_value = String::new();
        election.voters.clear(); // Clear the list of voters
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
