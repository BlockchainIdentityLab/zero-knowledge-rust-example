pub mod merkle {
    extern crate sha3;
    use self::sha3::{Digest, Sha3_256};
    use super::*;


    #[derive(Default)]
    pub struct MerkleTree {
    root: String,
    data: Vec<i32>,
    tree: Vec<String>
    }

    impl MerkleTree {
        pub fn from_vec(mut data: Vec<i32>) -> MerkleTree {
            // let mut hasher = Sha3_256::new();
            // let num = 11;
            // let result = hasher.result();

            let length: f64 = data.len() as f64;
            let next_pow_2 = 2_u32.pow(length.log2().ceil() as u32);

            data.resize(next_pow_2 as usize, 0);
            println!("{}", next_pow_2);
            println!("{:?}", data);

            let mut tree = Vec::new();
            tree.resize(next_pow_2 as usize, String::new());

            let mut hashed_data: Vec<String> = data.iter()   
                                  .map(|x| {
                                    let mut hasher = Sha3_256::new();
                                    hasher.input(x.to_string().as_bytes());
                                    let result = hasher.result();
                                    to_hex_string(&result.as_slice())
                                  })
                                  .collect();

            println!("{:?}", hashed_data);

            tree.append(&mut hashed_data);

            println!("{:?}", tree);

            for (i, value) in data.iter().enumerate().rev() {
                println!("{}", i);
                let mut hasher = Sha3_256::new();
                // convert to &[u8]
                hasher.input((tree[i*2] + tree[i*2 + 1]);
                // tree[i] = 
            }

            MerkleTree {
                root: String::from("ROOT"),
                data,
                tree: vec![String::from("s")]
            }
        }

    }

}

pub fn to_hex_string(bytes: &[u8]) -> String {
  let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02X}", b))
                               .collect();
  strs.join("")
}


#[cfg(test)]
mod tests { 
    use super::*;

    #[test] 
    pub fn test_merkle_from_vec() {
        let data: Vec<i32> = vec![-12, 0, 22, 12,2,4];
        merkle::MerkleTree::from_vec(data);
    }
}