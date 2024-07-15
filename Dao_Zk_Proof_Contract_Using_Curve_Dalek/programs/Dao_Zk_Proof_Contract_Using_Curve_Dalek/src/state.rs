use anchor_lang::prelude::*;

#[account]
pub struct Election {
    pub id: u64, // Unique identifier for the proposal
    pub token: Pubkey,
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

#[account]
pub struct ChangableTokenAccount {
    pub name: String,
    pub symbol: String,
    pub balance: u64,
}

#[account]
pub struct User {
    pub pubkey: Pubkey,
    pub reward_points: u64,
}