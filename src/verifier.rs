extern crate rand;

use rand::{thread_rng, Rng, SeedableRng, StdRng};

use prover;


// Any verifier can take the proofs provided by the prover and verify correctness
// Ie that the prover knows the solution to the partition problem for the given instance
pub fn verify_proof(problem: Vec<i64>, proofs: Vec<prover::Proof>) -> bool {
    let mut proof_checks_out: bool = true;
    let mut randomness_seed: Vec<String> = problem.iter()
                                            .map(|x| x.to_string())
                                            .collect::<Vec<String>>();

    // iterate through each proof
    // validate against problem and commitments
    for proof in proofs.iter() {
        let seed = prover::generate_seed(&randomness_seed);
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        let query_idx = rng.gen_range(0, problem.len());
        let merkle_root: String = (&proof.root).to_string();

        
        proof_checks_out = proof_checks_out && query_idx == proof.query_idx;

        // Test witness properties
        let abs_val = (proof.merkle_proof_1.value - proof.merkle_proof_2.value).abs();
        let abs_problem = problem[query_idx].abs();

        // Check |problem[i]|=|witness[i+1 % problem.len()]−witness[i]|
        proof_checks_out = proof_checks_out && (proof.merkle_proof_2.value - proof.merkle_proof_1.value).abs() == problem[query_idx].abs(); 

        // Authenticate paths in merkle trees to check commitments to witness match witness values
        proof_checks_out = proof_checks_out && proof.merkle_proof_1.verify_proof(&merkle_root, proof.query_idx as u32, problem.len() as u32);
        proof_checks_out = proof_checks_out && proof.merkle_proof_2.verify_proof(&merkle_root, ((proof.query_idx + 1) % problem.len()) as u32, problem.len() as u32); 

        // add string representation to of proof to randomness seed
        randomness_seed.push(format!("{:?}", &proof));

    }

    println!("Is proof valid : {}", proof_checks_out);
    proof_checks_out
}
