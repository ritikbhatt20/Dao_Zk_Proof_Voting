use anchor_lang::prelude::*;

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
    #[msg("Proof deserialization failed.")]
    ProofDeserializationFailed,
    #[msg("Invalid public input.")]
    InvalidPublicInput,
    #[msg("Proof verification failed.")]
    ProofVerificationFailed,
}
