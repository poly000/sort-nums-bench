# sort-nums-bench

```rust
let mut v = iter.collect::<Vec<_>>()
v.sort_unstable();
```

v.s.

```rust
let s = iter.collect::<BTreeSet<_>>();
```

The former is significantly faster (unless input are mostly `Eq`), because:

1. Rust's `sort_unstable` sorts items in-place, avoids bunch of heap allocations, at the cost of stablity.
2. Constructing a BTreeSet however invokes lots of heap allocations

So while the worst time complexity of BTreeMap `insert`s is `O(n log n)`,
the best time complexity of in-place merge sort is `O(n log n)` (`O(n²)` for the worst case),
in practice the number of heap allocations, or the _constant_ in other words,
results `collect::<Vec<_>> + sort_unstable` to be much faster than `collect::<BTreeSet<_>>`.

## results

Environment:

```text
Kernel: 6.7.6-zen1-1-zen
CPU: 12th Gen Intel i7-12700H (20) @ 4.600GHz
Memory: DDR5 8GB Dual-Channel (M425R1GB4BB0-CQKOL)
```

`RAND_NUMS=10000000`

mimalloc:
```
test tests::bench_btree_collect_search     ... bench:          64 ns/iter (+/- 44)
test tests::bench_vec_sort_unstable_search ... bench:          45 ns/iter (+/- 0)
```
tcmalloc:
```
test tests::bench_btree_collect_search     ... bench:          86 ns/iter (+/- 0)
test tests::bench_vec_sort_unstable_search ... bench:          45 ns/iter (+/- 0)
```
jemalloc:
```
test tests::bench_btree_collect_search     ... bench:          73 ns/iter (+/- 0)
test tests::bench_vec_sort_unstable_search ... bench:          52 ns/iter (+/- 0)
```
snmalloc:
```
test tests::bench_btree_collect_search     ... bench:          81 ns/iter (+/- 0)
test tests::bench_vec_sort_unstable_search ... bench:          48 ns/iter (+/- 0)
```
ferroc:
```
test tests::bench_btree_collect_search     ... bench:          89 ns/iter (+/- 0)
test tests::bench_vec_sort_unstable_search ... bench:          45 ns/iter (+/- 0)
```
rpmalloc:
```
test tests::bench_btree_collect_search     ... bench:          87 ns/iter (+/- 0)
test tests::bench_vec_sort_unstable_search ... bench:          43 ns/iter (+/- 1)
```
ptmalloc:
```
test tests::bench_btree_collect_search     ... bench:          66 ns/iter (+/- 0)
test tests::bench_vec_sort_unstable_search ... bench:          44 ns/iter (+/- 1)
```

