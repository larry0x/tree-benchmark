use {
    merk::Merk,
    std::{fs, io, path::Path, time::Instant},
    tree::Batch,
};

const DB_PATH: &str = "../testdata/db/merk";

fn bench_write(merk: &mut Merk, batches: Vec<Vec<merk::BatchEntry>>) {
    let start_time = Instant::now();
    let mut num_writes = 0;
    for batch in batches {
        num_writes += batch.len();
        unsafe {
            merk.apply_unchecked(&batch, &[]).unwrap();
        }
    }
    let elapsed_time = start_time.elapsed();

    println!("elapsed time:  {elapsed_time:?}");
    println!("num writes:    {num_writes}");
    println!("writes/second: {}", num_writes as f64 / elapsed_time.as_secs_f64());
}

fn bench_read(merk: &Merk, keys: &[Vec<u8>]) {
    let start_time = Instant::now();
    for key in keys {
        let _ = merk.get(&key).unwrap();
    }
    let elapsed_time = start_time.elapsed();

    println!("elapsed time: {elapsed_time:?}");
    println!("num reads:    {}", keys.len());
    println!("reads/second: {}", keys.len() as f64 / elapsed_time.as_secs_f64());
}

fn main() {
    let batches_bin = fs::read("../testdata/batches.json").unwrap();
    let batches: Vec<Batch<String, String>> = serde_json::from_slice(&batches_bin).unwrap();

    let keys_bin = fs::read("../testdata/keys.json").unwrap();
    let keys: Vec<String> = serde_json::from_slice(&keys_bin).unwrap();

    // convert tree::Batch to merk::Batch
    let batches = batches
        .into_iter()
        .map(|batch| {
            batch
                .into_iter()
                .map(|(key, op)| {
                    let merk_op = match op {
                        tree::Op::Insert(value) => merk::Op::Put(value.into_bytes()),
                        tree::Op::Delete => merk::Op::Delete,
                    };
                    (key.into_bytes(), merk_op)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // convert keys to Vec<u8>
    let keys = keys
        .into_iter()
        .map(|key| key.into_bytes())
        .collect::<Vec<_>>();

    if let Err(err) = fs::remove_dir_all(DB_PATH) {
        if err.kind() != io::ErrorKind::NotFound {
            panic!("{err}");
        }
    };
    let mut merk = Merk::open(Path::new(DB_PATH)).unwrap();

    println!("[1] writing to disk");
    bench_write(&mut merk, batches);

    println!("[2] reading from disk");
    bench_read(&merk, &keys);
}
