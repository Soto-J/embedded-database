# Embedded Database

A Rust embedded database project supporting both `std` and `no_std` environments, designed for practice and performance comparison.

## Features

- **Dual compilation modes**:
  - `no_std` by default (for embedded systems)
  - `std` feature flag for standard library support
- **Multiple storage backends**:
  - JSON-based storage (HashMap and BTreeMap)
  - Binary storage with bincode (HashMap and BTreeMap)
- **Performance benchmarks** comparing different storage implementations

## Project Structure

```
src/
├── database.rs          # Database struct with feature-gated storage
├── domain/
│   ├── error.rs        # Error types
│   └── mock_data.rs    # User type (std and no_std versions)
└── storage/
    ├── core/           # no_std implementations (in progress)
    └── std/            # std implementations
        ├── binary/     # Bincode serialization
        └── json/       # JSON serialization
```

## Running Benchmarks

To run the performance benchmarks, you must enable the `std` feature:

```bash
cargo bench --features std
```

**Note**: The benchmarks require the standard library and will not compile without the `std` feature flag.

## Building

### With standard library (default for testing):
```bash
cargo build --features std
cargo test --features std
```

### For embedded/no_std (default):
```bash
cargo build
```

## Benchmark Results

The benchmarks compare:
- HashMap vs BTreeMap performance
- JSON vs Binary (bincode) serialization (planned)
- std vs no_std overhead (planned)

Operations benchmarked:
- Insert (1k records)
- Get (1k lookups)
- Delete (1k removals)

## Future Goals

- [ ] Complete `no_std` storage implementations in `storage/core/`
- [ ] Add ESP32 target support
- [ ] Compare bincode vs JSON in both std and no_std
- [ ] Simulate embedded constraints (16KB heap, fixed-capacity collections)
