extern crate rand;
extern crate sha3;

use rand::{thread_rng, Rng, SeedableRng, StdRng};
use std::cmp;
use self::sha3::{Digest, Sha3_256};

use hashing;
use merkle;
use problem;

#[derive(Debug)]
// A proof for the response to a single interaction
// ie a proof the verifier can use to validate the witness at a specific query index
pub struct Proof {
    pub root: String,
    pub query_idx: usize,
    pub merkle_proof_1: merkle::MerkleProof,
    pub merkle_proof_2: merkle::MerkleProof
}


// A prover structure 
// can create a proof for a specific instance of the partition problem
pub struct Prover {
    pub problem: problem::PartictionProblem,
}

impl Prover {
    pub fn new(problem: problem::PartictionProblem) -> Prover {
        Prover {
            problem
        }
    }

    // generate a non interactive zero knowledge proof for the provers problem
    // proof simulates an interactive protocol by seeding a random generator
    // this seed is the hash of the problem plus and proof responses already generated
    // This means the verifier can replicate the random seed and hence verify the proof
    // num_queries is the number of interactions the proof simulates
    pub fn get_proof(&self, num_queries: u32) -> Vec<Proof> {
        let mut overall_proof: Vec<Proof> = Vec::new();

        let mut randomness_seed: Vec<String> = self.problem.partition_sizes.iter()
                                                    .map(|x| x.to_string())
                                                    .collect::<Vec<String>>();


        for mut i in 0..num_queries {
            let witness: Vec<i32> = self.get_witness();
            let tree: merkle::ZkMerkleTree = merkle::ZkMerkleTree::from_vec(&witness);

            let seed = generate_seed(&randomness_seed);
            let mut rng: StdRng = SeedableRng::from_seed(seed);
            let query_idx = rng.gen_range(0, self.problem.partition_sizes.len() + 1);

            let current_proof: Proof = Proof {
                root: String::from(tree.get_root()),
                query_idx,
                merkle_proof_1: tree.get_merkle_proof(query_idx),
                merkle_proof_2: tree.get_merkle_proof((query_idx + 1) % witness.len())
            };

            randomness_seed.push(format!("{:?}", &current_proof));
            overall_proof.push(current_proof);
        }

        overall_proof
    }

    // Given an instance of a partition problem via a list of numbers (the problem) and a list of
    // (-1, 1), we say that the assignment satisfies the problem if their dot product is 0.
    // The witness for the partition problem uses the fact the |l[i]|=|p[i+1]−p[i]| or if 
    pub fn get_witness(&self) -> Vec<i32> {

        let mut sum: i32 = 0;
        let mut mx = 0;


        // Witness starts with 0
        let mut witness: Vec<i32> = Vec::new();
        witness.push(sum);

        // randomly generate a either 1 or -1 for the side_obfuscator
        let mut rng = thread_rng();
        let r: i32 = rng.gen_range(0,2);
        let side_obfuscator: i32 = 1 - (2 * r);


        let iter = self.problem.partition_sizes.iter().zip(self.problem.side_assignment.iter());


        for (i, (num, side)) in iter.enumerate() {
            if *side != 1 && *side != -1 {
                panic!("Assignment vector must be 1's or -1's");
            }
            sum += side * num * side_obfuscator;
            witness.push(sum);
            mx = cmp::max(mx, sum);

        }

        // Ensure randomness of witness by shifting each value by a random shift value
        let shift = rng.gen_range(0, mx+1);
        let mut shifted_witness: Vec<i32> = Vec::new();
        for x in witness {
            shifted_witness.push(x + shift);
        }
        
        println!("Creating new witness for \n Partition Sizes: {:?} \n Assignment : {:?}", &self.problem.partition_sizes, &self.problem.side_assignment);
        println!("New Witness : {:?}", &shifted_witness);
        shifted_witness
    }




}

// Creates an size 32 byte vector in order seed the random generator
pub fn generate_seed(seed_vector: &[String]) -> [u8; 32] {
    let mut seed_string: String = String::new();  
    for el in seed_vector {
        seed_string.push_str(el);
    }
    let randhash = hashing::generate_hash_string(seed_string);
    let mut seed = [0u8; 32];
    for (x, y) in randhash.as_bytes().iter().zip(seed.iter_mut()) {
        *y = *x;
    }
    seed
}


