use anchor_lang::prelude::*;
use curve25519_dalek::ristretto::{RistrettoPoint, CompressedRistretto};
pub use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::Identity;
pub use rand::rngs::OsRng;
use sha3::{Sha3_512, Digest};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ElGamalPubkey {
    pub point: [u8; 32],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ElGamalCiphertext {
    pub c1: [u8; 32],
    pub c2: [u8; 32],
}

pub fn generate_keypair() -> (ElGamalPubkey, Scalar) {
    let secret_key = Scalar::random(&mut OsRng);
    let public_key = &RistrettoPoint::default() * &secret_key;
    (ElGamalPubkey { point: public_key.compress().to_bytes() }, secret_key)
}

pub fn encrypt(vote: bool, public_key: &ElGamalPubkey) -> ElGamalCiphertext {
    let r = Scalar::random(&mut OsRng);
    let pk = CompressedRistretto::from_slice(&public_key.point).unwrap().decompress().unwrap();
    let c1 = (&RistrettoPoint::default() * &r).compress();
    let mut c2 = (&pk * &r).compress();
    if vote {
        c2 = (c2.decompress().unwrap() + RistrettoPoint::default()).compress();
    }
    ElGamalCiphertext {
        c1: c1.to_bytes(),
        c2: c2.to_bytes(),
    }
}

pub fn decrypt(ciphertext: &ElGamalCiphertext, private_key: &Scalar) -> bool {
    let c1 = CompressedRistretto::from_slice(&ciphertext.c1).unwrap().decompress().unwrap();
    let c2 = CompressedRistretto::from_slice(&ciphertext.c2).unwrap().decompress().unwrap();
    let shared_secret = &c1 * private_key;
    let message_point = c2 - shared_secret;
    message_point == RistrettoPoint::default()
}

pub fn generate_ballot_hash(proposal_id: u64, user_pubkey: Pubkey) -> [u8; 64] {
    let mut hasher = Sha3_512::new();
    hasher.update(proposal_id.to_le_bytes());
    hasher.update(user_pubkey.to_bytes());
    hasher.finalize().into()
}