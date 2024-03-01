# sort-nums-bench

```rust
let mut v = iter.collect::<Vec<_>>()
v.sort_unstable();
```

v.s.

```rust
let s = iter.collect::<BTreeSet<_>>();
```

The later is significantly faster (unless input are mostly `Eq`), because:

1. Rust's `sort_unstable` sorts items in-place, avoids bunch of heap allocations, at the cost of stablity.
2. Constructing a BTreeSet however invokes lots of heap allocations

So while the worst time complexity of BTreeMap `insert`s is `O(n log n)`,
the best time complexity of in-place merge sort is `O(n log n)` (`O(n²)` for the worst case),
in practice the number of heap allocations, or the _constant_ in other words,
results `collect::<Vec<_>> + sort_unstable` to be much faster than `collect::<BTreeSet<_>>`.
