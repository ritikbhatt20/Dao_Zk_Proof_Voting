use bellman::groth16::{create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof};
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bls12_381::Bls12;
use bls12_381::Scalar;
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use std::convert::TryInto;
use num_bigint::{BigInt, Sign};
use num_traits::{FromPrimitive, Num};

const BLS12_381_SCALAR_FIELD_ORDER: &str = "52435875175126190479447740508185965837690552500527637822603658699938581184512";

fn sha256_to_scalar(data: &[u8]) -> Result<Scalar, &'static str> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();

    let hash_bigint = BigInt::from_bytes_be(Sign::Plus, &hash);
    let prime_order = BigInt::from_str_radix(BLS12_381_SCALAR_FIELD_ORDER, 10)
        .map_err(|_| "Failed to parse prime order")?;
    let scalar_value = hash_bigint % &prime_order;

    let scalar_bytes: [u8; 32] = scalar_value.to_bytes_be().1.try_into().unwrap_or_else(|_| [0u8; 32]);

    let scalar_ctoption = Scalar::from_bytes(&scalar_bytes);
    if scalar_ctoption.is_some().unwrap_u8() == 1 {
        Ok(scalar_ctoption.unwrap())
    } else {
        Err("Failed to convert to scalar")
    }
}

#[derive(Clone)]
pub struct VotingCircuit {
    pub docker_sha_num: Scalar,
    pub json_input_num: Scalar,
    pub expected_sum: Scalar,
}

impl Circuit<Scalar> for VotingCircuit {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let docker_sha_var = cs.alloc(|| "docker sha", || Ok(self.docker_sha_num))?;
        let json_input_var = cs.alloc(|| "json input", || Ok(self.json_input_num))?;
        let expected_sum_var = cs.alloc(|| "expected sum", || Ok(self.expected_sum))?;

        let sum = cs.alloc(|| "sum", || {
            let mut tmp = self.docker_sha_num;
            tmp += &self.json_input_num;
            Ok(tmp)
        })?;

        cs.enforce(|| "sum constraint", |lc| lc + docker_sha_var + json_input_var, |lc| lc + CS::one(), |lc| lc + sum);
        cs.enforce(|| "expected sum constraint", |lc| lc + sum, |lc| lc + CS::one(), |lc| lc + expected_sum_var);

        Ok(())
    }
}

pub fn generate_proof(docker_sha: &str, json_input: &str) -> Result<(Vec<u8>, Vec<String>), String> {
    let docker_sha_num = sha256_to_scalar(docker_sha.as_bytes()).map_err(|e| e.to_string())?;
    let json_input_num = sha256_to_scalar(json_input.as_bytes()).map_err(|e| e.to_string())?;

    let expected_sum = docker_sha_num + json_input_num;

    let circuit = VotingCircuit {
        docker_sha_num,
        json_input_num,
        expected_sum,
    };

    let mut rng = OsRng;
    let params = generate_random_parameters::<Bls12, _, _>(circuit.clone(), &mut rng).map_err(|e| e.to_string())?;
    let pvk = prepare_verifying_key(&params.vk);

    let proof = create_random_proof(circuit, &params, &mut rng).map_err(|e| e.to_string())?;
    let mut proof_bytes = vec![];
    proof.write(&mut proof_bytes).map_err(|e| e.to_string())?;

    let public_inputs = vec![
        docker_sha_num.to_string(),
        json_input_num.to_string(),
        expected_sum.to_string(),
    ];

    Ok((proof_bytes, public_inputs))
}

fn main() {
    let docker_sha = "your_docker_sha_here";
    let json_input = "your_json_input_here";

    match generate_proof(docker_sha, json_input) {
        Ok((proof, public_inputs)) => {
            println!("Proof: {:?}", proof);
            println!("Public Inputs: {:?}", public_inputs);
        }
        Err(e) => {
            println!("Error generating proof: {}", e);
        }
    }
}
