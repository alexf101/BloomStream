use std::fmt::Debug;

use clap::{command, Parser};
use ahash;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // How much memory, in bits, to allocate for each bloom filter
    #[arg(short, long)]
    filter_size: u64,

    /// Probability of writing to the next bloom filter
    #[arg(short, long, default_value_t = 0.1)]
    pass_through_prob: f32,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    let bf: &dyn BloomFilter = &FixedVectorFilter::new(args.filter_size as usize);
    println!("{:?}", bf);
}

trait BloomFilter: Debug + Sync {
    /// Insert a key into the bloom filter, returning true if the hashed key was already present, false otherwise.
    fn insert(&mut self, key: &str) -> bool;
    /// Check if the hash of a key is present in the bloom filter without updating it. The original value might
    /// have been previously inserted if this returns true, and definitely hasn't been if this returns false.
    fn maybe_contains(&self, key: &str) -> bool;
}

#[derive(Debug)]
struct FixedVectorFilter {
    vector: Vec<bool>,
    hasher: ahash::RandomState,
}

impl FixedVectorFilter {
    /// Construct a new bloom filter with the given size.
    fn new(size: usize) -> Self {
        FixedVectorFilter {
            vector: vec![false; size],
            hasher: ahash::RandomState::new(),
        }
    }
    fn hashed_index(&self, key: &str) -> usize {
        let index = self.hasher.hash_one(key.as_bytes()) % (self.vector.len() as u64);
        index.try_into().expect("unexpected error: result of modulo is larger than usize")
    }
}

impl BloomFilter for FixedVectorFilter {
    fn insert(&mut self, key: &str) -> bool {
        let hashed_index = self.hashed_index(key);
        let prev = self.vector[hashed_index];
        self.vector[hashed_index] = true;
        prev
    }
    
    fn maybe_contains(&self, key: &str) -> bool {
        let hashed_index = self.hashed_index(key);
        self.vector[hashed_index]
    }
}