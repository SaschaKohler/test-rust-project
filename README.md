# RealBench Rust Test Project

Ein Rust-Testprogramm für RealBench Performance Profiling.

## Build

```bash
cargo build --release
```

## Mit Debug-Info für Profiling

```bash
RUSTFLAGS="-g" cargo build --release
```

## Run

```bash
cargo run --release
```

## Performance-Tests

1. **Compute-Intensive Task** - Sorting und Vec-Operationen
2. **Memory Stress Test** - 1000 Allokationen à 1MB
3. **Fibonacci Calculation** - Iterative Berechnung
4. **Cache-Unfriendly Traversal** - Shuffled Linked List
5. **Branch Misprediction Test** - Unvorhersagbare Branches
6. **Deep Recursion Test** - Tiefe Rekursion (200 Levels)
7. **Virtual Dispatch Test** - Trait Object Call Overhead
8. **STL Heavy Test** - BTreeMap/HashMap/HashSet Operationen
9. **Lock Contention Test** - Mutex/RwLock/Atomic Vergleich
10. **Memory Fragmentation Test** - Random Allokationen

## Dependencies

- `rand` - Für Zufallszahlen und Shuffling
