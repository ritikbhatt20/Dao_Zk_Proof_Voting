use anchor_lang::prelude::*;
use curve25519_dalek::ristretto::{RistrettoPoint, CompressedRistretto};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::Identity;
use rand::rngs::OsRng;
use sha3::{Sha3_512, Digest};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ElGamalPubkey {
    pub compressed_point: [u8; 32],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ElGamalCiphertext {
    pub component1: [u8; 32],
    pub component2: [u8; 32],
}

pub fn create_keypair() -> (ElGamalPubkey, Scalar) {
    let secret_scalar = Scalar::random(&mut OsRng);
    let public_point = &RistrettoPoint::default() * &secret_scalar;
    (
        ElGamalPubkey {
            compressed_point: public_point.compress().to_bytes(),
        },
        secret_scalar,
    )
}

pub fn perform_encryption(vote: bool, public_key: &ElGamalPubkey) -> ElGamalCiphertext {
    let random_scalar = Scalar::random(&mut OsRng);
    let decompressed_public_point = CompressedRistretto::from_slice(&public_key.compressed_point)
        .unwrap()
        .decompress()
        .unwrap();
    let component1 = (&RistrettoPoint::default() * &random_scalar).compress();
    let mut component2 = (&decompressed_public_point * &random_scalar).compress();
    if vote {
        component2 = (component2.decompress().unwrap() + RistrettoPoint::default()).compress();
    }
    ElGamalCiphertext {
        component1: component1.to_bytes(),
        component2: component2.to_bytes(),
    }
}

pub fn perform_decryption(ciphertext: &ElGamalCiphertext, private_scalar: &Scalar) -> bool {
    let decompressed_component1 = CompressedRistretto::from_slice(&ciphertext.component1)
        .unwrap()
        .decompress()
        .unwrap();
    let decompressed_component2 = CompressedRistretto::from_slice(&ciphertext.component2)
        .unwrap()
        .decompress()
        .unwrap();
    let shared_secret = &decompressed_component1 * private_scalar;
    let message_point = decompressed_component2 - shared_secret;
    message_point == RistrettoPoint::default()
}

pub fn create_ballot_hash(proposal_id: u64, user_pubkey: Pubkey) -> [u8; 64] {
    let mut hasher = Sha3_512::new();
    hasher.update(proposal_id.to_le_bytes());
    hasher.update(user_pubkey.to_bytes());
    hasher.finalize().into()
}
