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

## results

Environment:

```text
Kernel: 6.7.6-zen1-1-zen
CPU: 12th Gen Intel i7-12700H (20) @ 4.600GHz
Memory: DDR5 8GB Dual-Channel (M425R1GB4BB0-CQKOL)
```

`RAND_NUMS=1024`

mimalloc:
```
test tests::bench_btree_collect     ... bench:      31,213 ns/iter (+/- 293)
test tests::bench_vec_sort          ... bench:      26,919 ns/iter (+/- 129)
test tests::bench_vec_sort_unstable ... bench:       9,276 ns/iter (+/- 446)
```

tcmalloc:
```
test tests::bench_btree_collect     ... bench:      31,366 ns/iter (+/- 394)
test tests::bench_vec_sort          ... bench:      27,438 ns/iter (+/- 74)
test tests::bench_vec_sort_unstable ... bench:      10,006 ns/iter (+/- 110)
```

jemalloc:
```
test tests::bench_btree_collect     ... bench:      32,808 ns/iter (+/- 225)
test tests::bench_vec_sort          ... bench:      28,886 ns/iter (+/- 163)
test tests::bench_vec_sort_unstable ... bench:       9,695 ns/iter (+/- 220)
```

snmalloc:
```
test tests::bench_btree_collect     ... bench:      32,190 ns/iter (+/- 219)
test tests::bench_vec_sort          ... bench:      28,850 ns/iter (+/- 178)
test tests::bench_vec_sort_unstable ... bench:       8,971 ns/iter (+/- 155)
```

ferroc:
```
test tests::bench_btree_collect     ... bench:      33,395 ns/iter (+/- 145)
test tests::bench_vec_sort          ... bench:      29,255 ns/iter (+/- 146)
test tests::bench_vec_sort_unstable ... bench:       9,765 ns/iter (+/- 166)
```

rpmalloc:
```
test tests::bench_btree_collect     ... bench:      31,330 ns/iter (+/- 145)
test tests::bench_vec_sort          ... bench:      27,132 ns/iter (+/- 101)
test tests::bench_vec_sort_unstable ... bench:       9,201 ns/iter (+/- 326)
```

PtMalloc:
```
test tests::bench_btree_collect     ... bench:      33,449 ns/iter (+/- 275)
test tests::bench_vec_sort          ... bench:      27,019 ns/iter (+/- 121)
test tests::bench_vec_sort_unstable ... bench:       8,602 ns/iter (+/- 518)
```
