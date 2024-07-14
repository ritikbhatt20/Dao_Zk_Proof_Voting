use super::elgamal::{ElGamalPubkey, ElGamalCiphertext};
use anchor_lang::prelude::*;
use curve25519_dalek::ristretto::{RistrettoPoint, CompressedRistretto};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::Identity;
use merlin::Transcript;
use rand::rngs::OsRng;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct VoteProof {
    pub a1: [u8; 32],
    pub a2: [u8; 32],
    pub z1: [u8; 32],
    pub z2: [u8; 32],
}

pub fn generate_vote_proof(vote: bool, r: Scalar, public_key: &ElGamalPubkey) -> VoteProof {
    let g = RistrettoPoint::default();
    let y = CompressedRistretto::from_slice(&public_key.point).unwrap().decompress().unwrap();
    
    let v = if vote { Scalar::ONE } else { Scalar::ZERO };
    
    let w1 = Scalar::random(&mut OsRng);
    let w2 = Scalar::random(&mut OsRng);
    
    let a1 = &g * &w1;
    let a2 = (&y * &w1) + (&g * &w2);
    
    let mut transcript = Transcript::new(b"vote_proof");
    transcript.append_message(b"a1", a1.compress().as_bytes());
    transcript.append_message(b"a2", a2.compress().as_bytes());
    
    let mut challenge_bytes = [0u8; 64];
    transcript.challenge_bytes(b"challenge", &mut challenge_bytes);
    let c = Scalar::from_bytes_mod_order_wide(&challenge_bytes);
    
    let z1 = w1 + (c * r);
    let z2 = w2 + (c * v);
    
    VoteProof {
        a1: a1.compress().to_bytes(),
        a2: a2.compress().to_bytes(),
        z1: z1.to_bytes(),
        z2: z2.to_bytes(),
    }
}

pub fn verify_vote_proof(proof: &VoteProof, ciphertext: &ElGamalCiphertext, public_key: &ElGamalPubkey) -> bool {
    let g = RistrettoPoint::default();
    let y = CompressedRistretto::from_slice(&public_key.point).unwrap().decompress().unwrap();
    let c1 = CompressedRistretto::from_slice(&ciphertext.c1).unwrap().decompress().unwrap();
    let c2 = CompressedRistretto::from_slice(&ciphertext.c2).unwrap().decompress().unwrap();
    
    let a1 = CompressedRistretto::from_slice(&proof.a1).unwrap().decompress().unwrap();
    let a2 = CompressedRistretto::from_slice(&proof.a2).unwrap().decompress().unwrap();
    let z1 = Scalar::from_canonical_bytes(proof.z1).unwrap();
    let z2 = Scalar::from_canonical_bytes(proof.z2).unwrap();
    
    let mut transcript = Transcript::new(b"vote_proof");
    transcript.append_message(b"a1", &proof.a1);
    transcript.append_message(b"a2", &proof.a2);
    
    let mut challenge_bytes = [0u8; 64];
    transcript.challenge_bytes(b"challenge", &mut challenge_bytes);
    let c = Scalar::from_bytes_mod_order_wide(&challenge_bytes);
    
    (&g * &z1 == a1 + (&c1 * &c)) && ((&y * &z1) + (&g * &z2) == a2 + (&c2 * &c))
}