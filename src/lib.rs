#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use std::{collections::BTreeSet, ops::Range, sync::OnceLock};

    use rand::random;
    use test::Bencher;

    const NUMS_AMOUNT: &str = env!("RAND_NUMS");
    fn nums_amount() -> usize {
        NUMS_AMOUNT.parse().ok().unwrap()
    }

    fn random_range() -> Range<u32> {
        static RANGE: OnceLock<Range<u32>> = OnceLock::new();
        RANGE
            .get_or_init(|| {
                let upper = random::<u32>() / 2 + u32::MAX / 2;
                let lower = random::<u32>() / 2;
                lower..upper
            })
            .clone()
    }

    #[bench]
    fn bench_vec_sort_unstable_search(b: &mut Bencher) {
        use binary_range_search::search_by;
        let range = random_range();

        let rand_nums = (0..=nums_amount()).map(|_| random()).collect::<Vec<u32>>();
        let mut vec = rand_nums.into_iter().collect::<Vec<_>>();
        vec.sort_unstable();
        b.iter(|| search_by(&vec, range.clone(), |x, y| x < y));
    }

    #[bench]
    fn bench_btree_collect_search(b: &mut Bencher) {
        let rand_nums = (0..=nums_amount()).map(|_| random()).collect::<Vec<u32>>();
        let tree = rand_nums.iter().collect::<BTreeSet<_>>();
        let range = random_range();
        b.iter(|| tree.range(range.clone()));
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
