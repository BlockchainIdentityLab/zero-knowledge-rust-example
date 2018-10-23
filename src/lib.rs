pub mod hashing {
    extern crate sha3;
    use self::sha3::{Digest, Sha3_256};

    pub fn generate_hash_string(value: String) -> String {
        let mut hasher = Sha3_256::new();
        hasher.input(value.as_bytes());
        let result = hasher.result();
        to_hex_string(&result.as_slice())
    }

    pub fn to_hex_string(bytes: &[u8]) -> String {
        let strs: Vec<String> = bytes.iter()
                                    .map(|b| format!("{:02X}", b))
                                    .collect();
        strs.join("")
    }
}

pub mod merkle {
    extern crate sha3;
    use self::sha3::{Digest, Sha3_256};
    use super::hashing;


    #[derive(Default)]
    pub struct MerkleTree {
        pub root: String,
        pub data: Vec<i32>,
        // a vector of hashed strings making up the merkle tree where index 1 is the root and i*2 and i*2+1 are its children
        pub tree: Vec<String>
    }

    impl MerkleTree {
        // creates a merkle tree 
        pub fn from_vec(data: &[i32]) -> MerkleTree {
            let length: f64 = data.len() as f64;
            let next_pow_2 = 2_u32.pow(length.log2().ceil() as u32);

            let mut tree_data = data.to_vec();
            tree_data.resize(next_pow_2 as usize, 0);
            println!("{}", next_pow_2);
            println!("{:?}", tree_data);

            let mut tree = Vec::new();
            tree.resize(next_pow_2 as usize, String::new());

            let mut hashed_data: Vec<String> = tree_data.iter()   
                                  .map(|x| {
                                    hashing::generate_hash_string(x.to_string())
                                  })
                                  .collect();

            tree.append(&mut hashed_data);

            for (i, value) in tree_data.iter().enumerate().rev() {
                println!("{}", i);
                    if i != 0 {
                    let mut hasher = Sha3_256::new();
                    // convert to &[u8]
                    let mut hash = String::new();
                    hash.push_str(&tree[i*2]);
                    hash.push_str(&tree[i*2 + 1]);
                    tree[i] = hashing::generate_hash_string(hash);
                }

            }

            // Should use lifetimes?
            MerkleTree {
                root: tree[1].clone(),
                data: tree_data,
                tree
            }
        }

        pub fn get_root(self) -> String {
            self.root
        }

        pub fn get_val_and_path(&self, id: usize) -> (i32, Vec<String>) {
            let val = self.data[id];
            let mut auth_path: Vec<String> = Vec::new();
            let mut id: usize = id + self.data.len();
            println!("PRE ID {}", id);
            while id > 1 {
                // the other child - either id -1 or id + 1
                auth_path.push(self.tree[id ^ 1].clone());
                println!("{}", (id as f32 / 2.0));
                id = (id as f32 / 2.0).floor() as usize;
                println!("New ID: {}", id);
            }

            (val, auth_path)

        }

        

    }

    pub fn verify_merkle_path(root: String, value_id: u32, data_size: u32, value: i32, path: &[String]) -> bool {
        let mut current = hashing::generate_hash_string(value.to_string());
        let mut tree_node_id = value_id + 2_u32.pow((data_size as f64).log2().ceil() as u32);

        for sibling in path.iter() {
            if tree_node_id == 1 {
                break;
            }
            let mut hash = String::new();
            if tree_node_id % 2 == 0 {
                hash.push_str(&current);
                hash.push_str(&sibling);
                current = hashing::generate_hash_string(hash);
            }
            else {                                   
                hash.push_str(&sibling);
                hash.push_str(&current);
                current = hashing::generate_hash_string(hash);    
            }

            tree_node_id = (tree_node_id as f32 / 2.0).floor() as u32;
        }
        // should be at the root
        assert!(tree_node_id == 1);
        root == current
    }


}




#[cfg(test)]
mod tests { 
    use super::*;

    #[test] 
    pub fn test_merkle_from_vec() {
        let data: Vec<i32> = vec![-12, 0, 22, 12,2,4];
        let merkle: merkle::MerkleTree = merkle::MerkleTree::from_vec(&data);
        assert_eq!(merkle.tree.len(), merkle.data.len() *2);
        assert_eq!(merkle.tree[0], String::from(""));
    }

    #[test]
    pub fn test_get_val_and_path() {
        let data: Vec<i32> = vec![12, 0, 32, 12];
        let merkle_tree: merkle::MerkleTree = merkle::MerkleTree::from_vec(&data);

        let (val, path) = merkle_tree.get_val_and_path(2);
        assert_eq!(val, 32);

    }

    #[test]
    pub fn verify_merkle_path() {
        let data: Vec<i32> = vec![1, 0, 32, 12];
        let merkle_tree: merkle::MerkleTree = merkle::MerkleTree::from_vec(&data);

        let (val, path) = merkle_tree.get_val_and_path(2);
        
        let in_tree = merkle::verify_merkle_path(merkle_tree.root, 2, data.len() as u32, val, &path);

        assert_eq!(in_tree, true);

    }
}