extern crate rand;
extern crate sha3;
extern crate merkle;

use rand::prelude::*;
use std::error::Error;
use std::cmp;
use std::process;
use sha3::{Digest, Sha3_256};
use std::str;

fn main() {
    println!("Hello, world!");

    let mut hasher = Sha3_256::new();
    let num = 11;
    hasher.input(num.to_string().as_bytes());
    let result = hasher.result();


    println!("{:?}", to_hex_string(&result.as_slice()));


    let problem: Vec<i32> = vec![1, 9, 8, 0, 2, 2];
    let assignment: Vec<i32> = vec![1, -1, 1, 1, -1, 1];
    let config = PartictionProblemConfig::new(&problem, &assignment).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    // config.get_witness();

}

pub fn to_hex_string(bytes: &[u8]) -> String {
  let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02X}", b))
                               .collect();
  strs.join("")
}

struct PartictionProblemConfig {
    boxes_to_sort: Vec<i32>,
    side_assignment: Vec<i32>,
}

impl PartictionProblemConfig {
    fn new(problem: &[i32], assignment: &[i32]) -> Result<PartictionProblemConfig, &'static str>  {
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
    fn get_witness(self) -> Vec<i32> {

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


#[derive(Default)]
struct MerkleTree {
  root: String,
  data: Vec<i32>,
  tree: Vec<String>
}

impl MerkleTree {
    fn from_vec(data: Vec<i32>) -> MerkleTree {
        let length: f64 = data.len() as f64;
        let next_pow_2 = length.log2();
        // let padded_data = 

        MerkleTree {
            root: String::from("ROOT"),
            data,
            tree: vec![String::from("s")]
        }
    }

}


// struct Witness {
//     data: Vec<i32>,
//     tree: MerkleTree
// }

