extern crate rand;
use rand::{thread_rng, Rng};
use std::error::Error;
use std::cmp;

pub struct PartictionProblemConfig {
    boxes_to_sort: Vec<i32>,
    side_assignment: Vec<i32>,
}


impl PartictionProblemConfig {
    pub fn new(problem: &[i32], assignment: &[i32]) -> Result<PartictionProblemConfig, &'static str>  {
        if problem.len() != assignment.len() {
            return Err("Input vectors must be equal size to create witness");
        }
        Ok(PartictionProblemConfig {
            boxes_to_sort: problem.to_vec(),
            side_assignment: assignment.to_vec(),
        })
    }
    // Given an instance of a partition problem via a list of numbers (the problem) and a list of
    // (-1, 1), we say that the assignment satisfies the problem if their dot product is 0.
    pub fn get_witness(self) -> Vec<i32> {

        let mut sum: i32 = 0;
        let mut mx = 0;
        // thread_rng is often the most convenient source of randomness:
        let mut rng = thread_rng();
        let r: i32 = rng.gen_range(0,2);
        // either 1 or -1
        let side_obfuscator: i32 = 1 - (2 * r);

        let mut witness: Vec<i32> = Vec::new();
        witness.push(sum);

        let iter = self.boxes_to_sort.iter().zip(self.side_assignment.iter());

        for (i, (num, side)) in iter.enumerate() {
            if *side != 1 && *side != -1 {
                panic!("Assignment vector must be 1's or -1's");
            }
            println!("{}: ({}, {})", i, num, side);
            sum += side * num * side_obfuscator;
            witness.push(sum);
            mx = cmp::max(mx, sum);

        }
        println!("witness {:?}", witness);
        println!("max {}", mx);

        let shift = rng.gen_range(0, mx+1);
        let mut shifted_witness: Vec<i32> = Vec::new();
        for x in witness {
            shifted_witness.push(x + shift);
        }
        
        shifted_witness
        
    }
}


pub mod hashing;

pub mod merkle;


#[cfg(test)]
mod tests { 
    use ::merkle;

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

        let (val, path) = merkle_tree.get_val_and_path(2);
        println!("{:?}", path);
        assert_eq!(val, 32);

    }

    #[test]
    pub fn verify_merkle_path() {
        let data: Vec<i32> = vec![1, 0, 32, 12];
        let merkle_tree: merkle::ZkMerkleTree = merkle::ZkMerkleTree::from_vec(&data);
        println!("Data {:?}", merkle_tree.data);
        println!("Tree {:?}", merkle_tree.tree);
        let (val, path) = merkle_tree.get_val_and_path(2);
        println!("Path {:?}", &path);

        let in_tree = merkle::verify_merkle_path(merkle_tree.root, 2, data.len() as u32, val, &path);

        assert_eq!(in_tree, true);

    }
}