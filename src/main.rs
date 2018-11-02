extern crate zkpartition;
use std::process;

use zkpartition::{merkle, prover, problem};


// Zero Knowledge Partition Problem Protocol
// The verifier chooses a random 0≤i≤n
// If i=n
// the verifier asks the prover to provide p[0] and p[n]
// and checks that they are both 0.
// Otherwise, the verifier asks the prover to provide p[i]
// and p[i+1] and checks that indeed |l[i]|=|p[i+1]−p[i]| (recall that l is known to the verifier, as part of the claim made by the prover).
// In this case p is the witness and l is the problem
fn main() {

    let problem: Vec<i64> = vec![1, 9, 8, 0, 2, 2];
    let assignment: Vec<i64> = vec![1, -1, 1, 1, -1, 1];
    let config = problem::PartictionProblem::new(&problem, &assignment).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    // let witness: Vec<i64> = config.get_witness();
    // let witness_zk_merkle: merkle::ZkMerkleTree = merkle::ZkMerkleTree::from_vec(&witness);
}

