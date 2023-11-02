use {
    cosmwasm_std::Storage,
    std::{fs, io, time::Instant},
    tree::{Batch, Tree},
    tree_benchmark::DBWrapper,
};

const DB_PATH: &str = "testdata/db/tree";
const TREE: Tree<String, String> = Tree::new_default();

fn bench_write(store: &mut dyn Storage, batches: Vec<Batch<String, String>>) {
    let start_time = Instant::now();
    let mut num_writes = 0;
    for batch in batches {
        num_writes += batch.len();
        TREE.apply(store, batch).unwrap();
    }
    let elapsed_time = start_time.elapsed();

    println!("elapsed time: {elapsed_time:?}");
    println!("writes per second: {}", num_writes as f64 / elapsed_time.as_secs_f64());
}

fn bench_read(store: &dyn Storage, keys: &[String]) {
    let start_time = Instant::now();
    for key in keys {
        let _ = TREE.get(store, key, false, None).unwrap();
    }
    let elapsed_time = start_time.elapsed();

    println!("elapsed time: {elapsed_time:?}");
    println!("reads per second: {}", keys.len() as u64 / elapsed_time.as_secs());
}

fn main() {
    let batches_bin = fs::read("testdata/batches.json").unwrap();
    let batches: Vec<Batch<String, String>> = serde_json::from_slice(&batches_bin).unwrap();

    let keys_bin = fs::read("testdata/keys.json").unwrap();
    let keys: Vec<String> = serde_json::from_slice(&keys_bin).unwrap();

    if let Err(err) = fs::remove_dir_all(DB_PATH) {
        if err.kind() != io::ErrorKind::NotFound {
            panic!("{err}");
        }
    };
    let mut store = DBWrapper::open(DB_PATH).unwrap();

    println!("[1] writing to disk");
    bench_write(&mut store, batches);

    println!("[2] reading from disk");
    bench_read(&store, &keys);
}
