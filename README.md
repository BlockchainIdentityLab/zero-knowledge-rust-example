# zero-knowledge-rust-example

This is an Rust implemenation of the python zero knowledge tutorial available [here](http://www.shirpeled.com/2018/09/a-hands-on-tutorial-for-zero-knowledge.html)

It is a zero knowledge proof for the partition problem. ie given a set of different sized partitions can you prove you know the two partition sets that are of equal size and can you do so without revealing anything about your solution?

## To Run

1. First install [Rust](https://www.rust-lang.org/en-US/install.html)
2. Clone repo
3. cd zero-knowledge-rust-example
3. cargo build
4. cargo test

You can play around with the tests in lib.rs to try create different proofs.


