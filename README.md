# tree-benchmark

Benchmark for state commitment schemes: [Tree](https://github.com/larry0x/tree), [Merk](https://github.com/turbofish-org/merk), and [JMT](https://github.com/penumbra-zone/jmt).

We decided to make this a separate repo because Merk requires nightly Rust, so if we put these benchmarks in another repo, that repo needs to be entirely in nightly, which is undesirable.

## Results

Methodology:

- 2023 MacBook Pro M2 Pro
- Write: 1,000,000 random keys and values in 2,000 batches, each being a random alphanumeric string, split into multiple batches
- Read: 100,000 keys randomly sampled from the above set
- IAVL uses GoLevelDB backend with 50 MB cache
- Merk and Tree use RocksDB backend with default config

|                     | writes per second | reads per second |
| ------------------- | ----------------- | ---------------- |
| Merk                | 67,940            | 456,719          |
| Tree                | 15,047            | 18,792           |
| IAVL (w/o fastnode) | 11,164            | 19,836           |
| IAVL (w/ fastnode)  | 7,901             | 286,861          |

## How to use

Generate test data:

```bash
cargo run --bin generate-test-data
```

Run benchmarks:

```bash
# iavl
go run main.go

# merk
cargo run --release --bin bench-merk

# tree
cargo run --release --bin bench-tree
```
