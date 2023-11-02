//! Generate 2000 batches of data to write, each batch consists of 500 writes,
//! with each key or value been an alphanumeric string of lengths between 1-20,
//! and save it as a JSON file.
//!
//! For queries, we randomly pick 100,000 keys from those batches.

use {
    rand::Rng,
    random_string::charsets::ALPHANUMERIC,
    std::{collections::BTreeSet, fs},
    tree::{Batch, Op},
};

fn main() {
    let mut batches = vec![];
    let mut keys = BTreeSet::new();
    let mut rng = rand::thread_rng();

    println!("generating batches...");
    for _ in 1..=2000 {
        let mut batch = Batch::new();
        for _ in 1..=500 {
            let key_len = rng.gen_range(1..=20);
            let key = random_string::generate(key_len, ALPHANUMERIC);
            let value_len = rng.gen_range(1..=20);
            let value = random_string::generate(value_len, ALPHANUMERIC);
            batch.insert(key, Op::Insert(value));
        }

        batches.push(batch);
    }

    println!("generating keys...");
    while keys.len() < 100000 {
        let batch_idx = rng.gen_range(0..batches.len());
        let batch = &batches[batch_idx];

        let key_idx = rng.gen_range(0..batch.len());
        let key = batch.iter().nth(key_idx).unwrap();

        keys.insert(key.0.clone());
    }

    println!("serializing batches and save to file...");
    let batches_bin = serde_json::to_vec_pretty(&batches).unwrap();
    fs::write("../testdata/batches.json", batches_bin).unwrap();

    println!("serializing keys and save to file...");
    let keys_bin = serde_json::to_vec_pretty(&keys).unwrap();
    fs::write("../testdata/keys.json", keys_bin).unwrap();

    println!("done!");
}
