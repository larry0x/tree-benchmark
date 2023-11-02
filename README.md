# tree-benchmark

Benchmark for state commitment schemes: [Tree](https://github.com/larry0x/tree), [Merk](https://github.com/turbofish-org/merk), and [JMT](https://github.com/penumbra-zone/jmt).

We decided to make this a separate repo because Merk requires nightly Rust, so if we put these benchmarks in another repo, that repo needs to be entirely in nightly, which is undesirable.

## How to use

Generate test data:

```bash
cargo run --bin generate-test-data
```

Run benchmarks:

```bash
cargo run --release --bin bench-merk
cargo run --release --bin bench-tree
```

## Results

Methodology:

- 2023 MacBook Pro M2 Pro
- write: 1,000,000 random keys and values in 2,000 batches, each being a random alphanumeric string, split into multiple batches
- read: 100,000 keys randomly sampled from the above set

|      | writes per second | reads per second |
| ---- | ----------------- | ---------------- |
| Merk | 65,851            | 417,711          |
| Tree | 15,042            | 18,752           |
