#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use rand::random;
    use test::Bencher;

    const NUMS_AMOUNT: &str = env!("RAND_NUMS");
    fn nums_amount() -> usize {
        NUMS_AMOUNT.parse().ok().unwrap()
    }

    #[bench]
    fn bench_vec_sort(b: &mut Bencher) {
        let rand_nums = (0..=nums_amount()).map(|_| random()).collect::<Vec<u32>>();
        b.iter(|| {
            let mut vec = rand_nums.iter().collect::<Vec<_>>();
            vec.sort();
        });
    }

    #[bench]
    fn bench_vec_sort_unstable(b: &mut Bencher) {
        let rand_nums = (0..=nums_amount()).map(|_| random()).collect::<Vec<u32>>();
        b.iter(|| {
            let mut vec = rand_nums.iter().collect::<Vec<_>>();
            vec.sort_unstable();
        });
    }

    #[bench]
    fn bench_btree_collect(b: &mut Bencher) {
        let rand_nums = (0..=nums_amount()).map(|_| random()).collect::<Vec<u32>>();
        b.iter(|| {
            let _tree = rand_nums.iter().collect::<BTreeSet<_>>();
        });
    }
}

mod _alloc {
    #[cfg(feature = "ferroc")]
    use ferroc::Ferroc as Malloc;
    #[cfg(feature = "jemalloc")]
    use jemallocator::Jemalloc as Malloc;
    #[cfg(feature = "mimalloc")]
    use mimalloc::MiMalloc as Malloc;
    #[cfg(feature = "rpmalloc")]
    use rpmalloc::RpMalloc as Malloc;
    #[cfg(feature = "snmalloc")]
    use snmalloc_rs::SnMalloc as Malloc;
    #[cfg(not(feature = "alloc"))]
    use std::alloc::System as Malloc;
    #[cfg(feature = "tcmalloc")]
    use tcmalloc2::TcMalloc as Malloc;

    #[global_allocator]
    static GLOBAL: Malloc = Malloc;
}
