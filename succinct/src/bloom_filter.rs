use super::bv::BitVec;
use seahash::hash_seeded;
use rand::Rng;
use std::hash::{Hash, Hasher};

struct BlockedBloomFilter{
    bv: BitVec,
    k: usize,
    seeds: Vec<u64>,
    nb: usize,
    b_size: usize,
}

struct BloomFilter {
    bv: BitVec,
    k: usize,
    n: usize,
    seeds: Vec<u64>,
}

impl BlockedBloomFilter {
    pub fn new(k: usize, n_blocks: usize, block_size: usize) -> Self{
        let mut rng = rand::thread_rng();
        let n_seeds = (k + 1) * 4;
        let seeds = (0..n_seeds).map(|_x| rng.gen::<u64>()).collect();

        Self {
            bv: BitVec::new(n_blocks * block_size),
            k: k,
            nb: n_blocks,
            seeds: seeds,
            b_size: block_size,
        }
    }

    pub fn n_hashes(&self) -> usize {
        self.k
    }

    pub fn len(&self) -> usize {
        self.bv.len()
    }

    pub fn block_size(&self) -> usize {
        self.b_size
    }

    pub fn n_blocks(&self) -> usize {
        self.nb
    }

    pub fn insert<H: Hash>(&mut self, item: &H) {
        let n = self.len();
        let hx_block = self.hash_block(item);
        for i in 0..self.n_hashes() {
            let hx_in_block = self.hash_in_block(i, item);
            self.bv.set(hx_block + hx_in_block, true);
        }
    }

    pub fn query<H: Hash>(&self, item: &H) -> bool {
        let mut isin = true;
        let hx_block = self.hash_block(item);

        for i in 0..self.n_hashes() {
            let hx_in_block = self.hash_in_block(i, item);
            isin &= self.bv.get(hx_block + hx_in_block);
        }
        isin
    }

    fn hash_block<H: Hash>(&self, item: &H) -> usize {
        self.hash_i_mod(0, item, self.n_blocks()) * self.block_size()
    }

    fn hash_in_block<H: Hash>(&self, i: usize, item: &H) -> usize {
        self.hash_i_mod(i + 1, item, self.block_size())
    }


    fn hash_i_mod<H: Hash>(&self, i: usize, item: &H, m: usize) -> usize {
        let s_i = i * 4;
        let mut hasher = seahash::SeaHasher::with_seeds(self.seeds[s_i],
                                                        self.seeds[s_i + 1],
                                                        self.seeds[s_i + 2],
                                                        self.seeds[s_i + 3]);
        item.hash(&mut hasher);
        hasher.finish() as usize % m
    }
}


impl BloomFilter {
    pub fn new(k: usize, n: usize) -> Self{
        let mut rng = rand::thread_rng();
        let n_seeds = k * 4;
        let seeds = (0..n_seeds).map(|_x| rng.gen::<u64>()).collect();

        Self {
            bv: BitVec::new(n),
            k: k,
            n: n,
            seeds: seeds
        }
    }

    pub fn n_hashes(&self) -> usize {
        self.k
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn insert<H: Hash>(&mut self, item: &H) {
        let n = self.len();
        for i in 0..self.n_hashes() {
            self.bv.set(self.hash_i(i, item), true);
        }
    }

    pub fn query<H: Hash>(&self, item: &H) -> bool {
        let mut isin = true;
        for i in 0..self.n_hashes() {
            isin &= self.bv.get(self.hash_i(i, item));
        }
        isin
    }

    fn hash_i<H: Hash>(&self, i: usize, item: &H) -> usize {
        let s_i = i * 4;
        let mut hasher = seahash::SeaHasher::with_seeds(self.seeds[s_i],
                                                        self.seeds[s_i + 1],
                                                        self.seeds[s_i + 2],
                                                        self.seeds[s_i + 3]);
        item.hash(&mut hasher);
        hasher.finish() as usize % self.len()
    }
}

#[cfg(test)]
mod bf_tests {
    use crate::bloom_filter::*;

    #[test]
    fn new() {
        let k = 5;
        let n = 15;
        let bf = BloomFilter::new(k, n);
        assert_eq!(bf.len(), n);
        assert_eq!(bf.bv.len(), n);
        assert_eq!(bf.n_hashes(), k);
        assert_eq!(bf.seeds.len(), k*4);
    }

    #[test] 
    fn insert_query_bf() {
        let k = 10;
        let n = 100;
        let mut bf = BloomFilter::new(k, n);

        let s = b"hello";
        bf.insert(s);
        assert!(bf.query(s));

        let s = 73;
        assert!(!bf.query(&s));
    }

    #[test] 
    fn insert_query_bbf() {
        let k = 8;
        let b_size = 64;
        let nb = 100;
        let mut bf = BlockedBloomFilter::new(k, nb, b_size);

        let s = b"hello";
        bf.insert(s);
        assert!(bf.query(s));

        let s = 73;
        assert!(!bf.query(&s));
    }
}