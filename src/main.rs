extern crate zkpartition;
use std::process;

use zkpartition::merkle;

fn main() {

    let problem: Vec<i32> = vec![1, 9, 8, 0, 2, 2];
    let assignment: Vec<i32> = vec![1, -1, 1, 1, -1, 1];
    let config = zkpartition::PartictionProblemConfig::new(&problem, &assignment).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    let witness: Vec<i32> = config.get_witness();
    let witness_zk_merkle: merkle::ZkMerkleTree = merkle::ZkMerkleTree::from_vec(&witness);
}

