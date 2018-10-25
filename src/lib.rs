extern crate rand;
use rand::{thread_rng, Rng};
use std::error::Error;
use std::cmp;

pub mod prover;

pub mod hashing;

pub mod merkle;

pub mod problem;

pub mod verifier;

#[cfg(test)]
mod tests { 
    use ::{merkle, prover, problem, verifier};

    #[test] 
    pub fn test_merkle_from_vec() {
        let data: Vec<i32> = vec![-12, 0, 22, 12,2,4];
        let merkle: merkle::ZkMerkleTree = merkle::ZkMerkleTree::from_vec(&data);
        assert_eq!(merkle.tree.len(), merkle.data.len() *2);
        assert_eq!(merkle.tree[0], String::from(""));
    }

    #[test]
    pub fn test_get_val_and_path() {
        let data: Vec<i32> = vec![12, 0, 32, 12];
        let merkle_tree: merkle::ZkMerkleTree = merkle::ZkMerkleTree::from_vec(&data);

        let merkle_proof: merkle::MerkleProof = merkle_tree.get_merkle_proof(2);
        println!("{:?}", merkle_proof.authentication_path);
        assert_eq!(merkle_proof.value, 32);

    }

    #[test]
    pub fn verify_merkle_path() {
        let witness: Vec<i32> = vec![1, 0, 32, 12];
        let merkle_tree: merkle::ZkMerkleTree = merkle::ZkMerkleTree::from_vec(&witness);
        println!("Data {:?}", merkle_tree.data);
        println!("Tree {:?}", merkle_tree.tree);
        let merkle_proof: merkle::MerkleProof = merkle_tree.get_merkle_proof(2);
        println!("Path {:?}", &merkle_proof.authentication_path);

        let in_tree = merkle_proof.verify_proof(&merkle_tree.root, 2, witness.len() as u32);

        assert_eq!(in_tree, true);
    }

    #[test]
    pub fn verify_true_proof() {
        let problem: Vec<i32> = vec![1, 2, 3, 6, 6, 6, 12];
        let assignment: Vec<i32> = vec![1, 1, 1, -1, -1, -1, 1];
        let problem_config: problem::PartictionProblem = problem::PartictionProblem::new(&problem, &assignment).unwrap();
        let prover: prover::Prover = prover::Prover::new(problem_config);
        let proof: Vec<prover::Proof> = prover.get_proof(100);
        println!("Proof \n {:?}", proof);

        let is_proof_valid = verifier::verify_proof(problem, proof);

        assert!(is_proof_valid);
    }

    #[test]
    pub fn verify_false_proof() {
        let problem: Vec<i32> = vec![1, 2, 3, 6, 6, 6, 12];
        let assignment: Vec<i32> = vec![1, -1, -1, -1, -1, -1, 1];
        let problem_config: problem::PartictionProblem = problem::PartictionProblem::new(&problem, &assignment).unwrap();
        let prover: prover::Prover = prover::Prover::new(problem_config);
        let proof: Vec<prover::Proof> = prover.get_proof(100);
        // println!("Proof \n {:?}", proof);

        let is_proof_valid = verifier::verify_proof(problem, proof);
        assert!(!is_proof_valid);
    }
}