extern crate sha3;
extern crate rand;

use self::sha3::{Digest, Sha3_256};

use hashing;



#[derive(Default)]
// A Zero Knowledge Merkle tree implementation using SHA256
pub struct ZkMerkleTree {
    pub root: String,
    pub data: Vec<i32>,
    // a vector of hashed strings making up the merkle tree where index 1 is the root and i*2 and i*2+1 are its children
    pub tree: Vec<String>
}

impl ZkMerkleTree {
    // creates a zero knowledge merkle tree from a vector
    // could extend to use Generics
    pub fn from_vec(data: &[i32]) -> ZkMerkleTree {

        // resize data array to make full binary tree leafs ie next power of 2
        let length: f64 = data.len() as f64;
        let next_pow_2 = 2_u32.pow(length.log2().ceil() as u32);
        let mut base_data = data.to_vec();
        base_data.resize(next_pow_2 as usize, 0);

        // create vecotr of random i32 the same size as the leaf nodes
        let mut random_list: Vec<i32> = Vec::new();
        for i in 0..next_pow_2 {
            let random: i32 = rand::random();
            random_list.push(random);
        }

        // Intertwine with randomness with data to obtain zero knowledge.
        let mut leaf_node_data: Vec<i32> = Vec::new();
        for (data, random) in base_data.iter().zip(random_list.iter()) {
            leaf_node_data.push(*data);
            leaf_node_data.push(*random);
        }

        let mut tree = Vec::new();
        // resize actual tree vector to account higher layers of merkle tree 
        tree.resize((next_pow_2 * 2) as usize, String::new());

        // hash each of the leaf nodes
        let mut hashed_leaf_data: Vec<String> = leaf_node_data.iter()   
                                .map(|x| {
                                hashing::generate_hash_string(x.to_string())
                                })
                                .collect();
        // Add hashed leaf node data to end of tree vector
        tree.append(&mut hashed_leaf_data);

        // Merkle Tree node at position i equal to the hash of tree node at i*2 + tree node at i*2 + 1
        for (i, value) in leaf_node_data.iter().enumerate().rev() {
            // no node at tree[0] for simplicity later
            if i != 0 {
                let mut hasher = Sha3_256::new();                   
                let mut hash = String::new();
                hash.push_str(&tree[i*2]);
                hash.push_str(&tree[i*2 + 1]);
                tree[i] = hashing::generate_hash_string(hash);
            }

        }

        println!("The Zk Merkle Tree \n {:?}", &tree);
        // Should use lifetimes?
        ZkMerkleTree {
            root: tree[1].clone(),
            data: leaf_node_data,
            tree
        }

    }

    pub fn get_root(self) -> String {
        self.root
    }


    pub fn get_val_and_path(&self, id: usize) -> (i32, Vec<String>) {
        // Because of the zk padding, the data is now at id * 2
        let mut index = id * 2;
        let val = self.data[index];
        let mut auth_path: Vec<String> = Vec::new();
        index = index + self.data.len();

        while index > 1 {
            // the other child - either id -1 or id + 1 => (index ^ 1)
            auth_path.push(self.tree[index ^ 1].clone());
            index = (index as f32 / 2.0).floor() as usize;
        }
        println!("Value at data index {} is {} with Path \n {:?} ", id, val, &auth_path);
        (val, auth_path)

    }

    

}

// validates the data is in the path of a merkle tree by hashing the data against each of the path hashes in turn
pub fn verify_merkle_path(root: String, value_id: u32, data_size: u32, value: i32, path: &[String]) -> bool {
    let mut current: String = hashing::generate_hash_string(value.to_string());
    // Due to zk padding, data_size needs to be multiplied by 2, as does the value_id
    let mut tree_node_id = value_id * 2 + 2_u32.pow(((data_size * 2) as f64).log2().ceil() as u32);

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
    // should be at the root node
    assert!(tree_node_id == 1);
    root == current
}