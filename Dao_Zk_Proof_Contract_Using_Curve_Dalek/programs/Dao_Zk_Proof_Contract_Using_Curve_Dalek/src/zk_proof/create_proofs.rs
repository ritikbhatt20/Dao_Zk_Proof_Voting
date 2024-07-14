use super::elgamal::{ElGamalPubkey, ElGamalCiphertext};
use anchor_lang::prelude::*;
use curve25519_dalek::ristretto::{RistrettoPoint, CompressedRistretto};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::Identity;
use merlin::Transcript;
use rand::rngs::OsRng;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct VoteProof {
    pub proof_component1: [u8; 32],
    pub proof_component2: [u8; 32],
    pub response1: [u8; 32],
    pub response2: [u8; 32],
}

pub fn create_vote_proof(vote: bool, random_scalar: Scalar, public_key: &ElGamalPubkey) -> VoteProof {
    let generator_point = RistrettoPoint::default();
    let decompressed_public_point = CompressedRistretto::from_slice(&public_key.compressed_point)
        .unwrap()
        .decompress()
        .unwrap();

    let vote_scalar = if vote { Scalar::ONE } else { Scalar::ZERO };

    let random_response1 = Scalar::random(&mut OsRng);
    let random_response2 = Scalar::random(&mut OsRng);

    let proof_component1_point = &generator_point * &random_response1;
    let proof_component2_point = (&decompressed_public_point * &random_response1) + (&generator_point * &random_response2);

    let mut transcript = Transcript::new(b"vote_proof");
    transcript.append_message(b"proof_component1", proof_component1_point.compress().as_bytes());
    transcript.append_message(b"proof_component2", proof_component2_point.compress().as_bytes());

    let mut challenge_bytes = [0u8; 64];
    transcript.challenge_bytes(b"challenge", &mut challenge_bytes);
    let challenge_scalar = Scalar::from_bytes_mod_order_wide(&challenge_bytes);

    let response1 = random_response1 + (challenge_scalar * random_scalar);
    let response2 = random_response2 + (challenge_scalar * vote_scalar);

    VoteProof {
        proof_component1: proof_component1_point.compress().to_bytes(),
        proof_component2: proof_component2_point.compress().to_bytes(),
        response1: response1.to_bytes(),
        response2: response2.to_bytes(),
    }
}

pub fn validate_vote_proof(proof: &VoteProof, ciphertext: &ElGamalCiphertext, public_key: &ElGamalPubkey) -> bool {
    let generator_point = RistrettoPoint::default();
    let decompressed_public_point = CompressedRistretto::from_slice(&public_key.compressed_point)
        .unwrap()
        .decompress()
        .unwrap();
    let decompressed_component1 = CompressedRistretto::from_slice(&ciphertext.component1)
        .unwrap()
        .decompress()
        .unwrap();
    let decompressed_component2 = CompressedRistretto::from_slice(&ciphertext.component2)
        .unwrap()
        .decompress()
        .unwrap();

    let decompressed_proof_component1 = CompressedRistretto::from_slice(&proof.proof_component1)
        .unwrap()
        .decompress()
        .unwrap();
    let decompressed_proof_component2 = CompressedRistretto::from_slice(&proof.proof_component2)
        .unwrap()
        .decompress()
        .unwrap();
    let response1_scalar = Scalar::from_canonical_bytes(proof.response1).unwrap();
    let response2_scalar = Scalar::from_canonical_bytes(proof.response2).unwrap();

    let mut transcript = Transcript::new(b"vote_proof");
    transcript.append_message(b"proof_component1", &proof.proof_component1);
    transcript.append_message(b"proof_component2", &proof.proof_component2);

    let mut challenge_bytes = [0u8; 64];
    transcript.challenge_bytes(b"challenge", &mut challenge_bytes);
    let challenge_scalar = Scalar::from_bytes_mod_order_wide(&challenge_bytes);

    (&generator_point * &response1_scalar == decompressed_proof_component1 + (&decompressed_component1 * &challenge_scalar)) &&
    ((&decompressed_public_point * &response1_scalar) + (&generator_point * &response2_scalar) == decompressed_proof_component2 + (&decompressed_component2 * &challenge_scalar))
}
