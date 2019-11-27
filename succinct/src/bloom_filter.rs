use super::bv::BitVec;
use super::math::cdiv;

use seahash::hash_seeded;
use rand::Rng;
use std::hash::{Hash, Hasher};
use std::f32::consts::LN_2;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockedBloomFilter{
    bv: BitVec,
    k: usize,
    seeds: Vec<u64>,
    nb: usize,
    b_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloomFilter {
    bv: BitVec,
    k: usize,
    n: usize,
    seeds: Vec<u64>,
}

pub trait MQ {
    /* Membership Query */
    fn insert<H: Hash>(&mut self, item: &H);
    fn query<H: Hash>(&self, item: &H) -> bool;
}

impl MQ for BlockedBloomFilter {
    fn insert<H: Hash>(&mut self, item: &H) {
        let n = self.len();
        let hx_block = self.hash_block(item);
        for i in 0..self.n_hashes() {
            let hx_in_block = self.hash_in_block(i, item);
            self.bv.set(hx_block + hx_in_block, true);
        }
    }

    fn query<H: Hash>(&self, item: &H) -> bool {
        let mut isin = true;

        // First hash is used for block
        let hx_block = self.hash_block(item);

        // next n-1 hashes are used 
        for i in 1..self.n_hashes() {
            let hx_in_block = self.hash_in_block(i, item);
            isin &= self.bv.get(hx_block + hx_in_block);
            if !isin {
                return isin;
            }
        }
        isin
    }
}

impl MQ for BloomFilter {
    fn insert<H: Hash>(&mut self, item: &H) {
        let n = self.len();
        for i in 0..self.n_hashes() {
            self.bv.set(self.hash_i(i, item), true);
        }
    }

    fn query<H: Hash>(&self, item: &H) -> bool {
        let mut isin = true;
        for i in 0..self.n_hashes() {
            isin &= self.bv.get(self.hash_i(i, item));
            if !isin {
                return isin;
            }
        }
        isin
    }
}


impl BlockedBloomFilter {
    pub fn new(k: usize, n_blocks: usize, block_size: usize) -> Self{
        // block_size in bytes

        let mut rng = rand::thread_rng();
        let n_seeds = k * 4; //TODO: or is it k+1?
        let seeds = (0..n_seeds).map(|_x| rng.gen::<u64>()).collect();

        Self {
            bv: BitVec::new(n_blocks * block_size * 8),
            k: k,
            nb: n_blocks,
            seeds: seeds,
            b_size: block_size * 8,
        }
    }

    pub fn with_fpr(fpr: f32, n: usize, block_size: usize) -> Self{
        /* Create BF with fp rate `fpr` and `n` expected elements */
        let (k, m) = bf_with_fpr_config(fpr, n);

        let n_blocks = cdiv(m, block_size * 8);

        Self::new(k, n_blocks, block_size)
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


    fn hash_block<H: Hash>(&self, item: &H) -> usize {
        self.hash_i_mod(0, item, self.n_blocks()) * self.block_size()
    }

    fn hash_in_block<H: Hash>(&self, i: usize, item: &H) -> usize {
        self.hash_i_mod(i, item, self.block_size())
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

    pub fn with_fpr(fpr: f32, n: usize) -> Self{
        /* Create BF with fp rate `fpr` and `n` expected elements */

        let (k, m) = bf_with_fpr_config(fpr, n);

        Self::new(k, m)
    }

    pub fn n_hashes(&self) -> usize {
        self.k
    }

    pub fn len(&self) -> usize {
        self.n
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

fn bf_with_fpr_config(fpr: f32, n: usize) -> (usize, usize) {
    // 1) Calculate optimal size:
    let m = -1.0 * n as f32 * fpr.ln() / (LN_2 * LN_2);

    // 2) Calculate optimal k
    let k = ((m as f32 / n as f32) * LN_2).ceil();

    // recalculate optimal size since we ceil k
    let m_ceil = (k * n as f32 / LN_2).ceil();

    (k as usize, m_ceil as usize)
}

#[cfg(test)]
mod bf_tests {
    use crate::bloom_filter::*;
    use crate::bloom_filter::MQ;

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

    #[test]
    fn bf_from_fpr() {
        // Sanity check for FPR
        // fpr = 50%, expected 10 items.

        let mut bf = BloomFilter::with_fpr(0.5, 10);

        assert_eq!(bf.len(), 15);
        assert_eq!(2, bf.n_hashes());
    }

    fn bf_fpr_sanity() {
        // Sanity check for FPR
        // fpr = 50%, expected 10 items.

        let mut bf = BloomFilter::with_fpr(0.9, 1000);

        for i in 0..900 {
            bf.insert(&i);
        }

        let mut n_fp = 0;
        for i in 900..1000 {
            if bf.query(&i) {
                n_fp += 1;
            }
        }
        
        println!("fpr: {}", n_fp);
        assert!(n_fp <= 90);
    }
}