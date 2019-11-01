use super::bv::{IntVec, BitVec};
use std::cmp::{min, max};

#[derive(Debug)]
pub struct RankSupport<'a> {
    bv: &'a BitVec,
    s: usize, // Probably can be smaller uint
    b: usize, 
    rs: IntVec,
    rb: IntVec,
}

impl<'a> RankSupport<'a> {
    
    pub fn new (bv: &'a BitVec) -> Self {
        let s = max(cdiv_2(clog(bv.len())*clog(bv.len())), 1);
        let b = max(cdiv_2(clog(bv.len())), 1);
        let s = (s / b) * b; // hack to make s divisible! no more book keeping...
        assert_eq!(s % b, 0);
        let rs = Self::get_rs(&bv, s);
        let rb = Self::get_rb(&bv, s,  b);
        Self {
                bv: &bv, //? is there a better way to do this?
                s: s,
                b: b,
                rs: rs,
                rb: rb,
            }
    }

    fn  get_rs(bv: &BitVec, s: usize) -> IntVec{
        let n = bv.len();
        let n_blocks = cdiv(n, s);
        let w = max(clog(n), 1); //we still need 1 bit to store "0"
        let mut rs = IntVec::new(w, n_blocks);
        let mut count = 0;
        for i in 0..(n_blocks - 1) {
            // then count each 32 bit chunk...
            let mut counted_bits = 0;
            while counted_bits < s {
                let bits_to_count = min(32, s - counted_bits);
                count += bv.get_int(i*s + counted_bits, bits_to_count).count_ones();
                counted_bits += bits_to_count;
            }
            rs.set_int(i + 1, count);
        }
        rs
    }

    fn get_rb(bv: &BitVec, s: usize, b:usize) -> IntVec {
        let n = bv.len();
        let n_bblocks = cdiv(n, b);
        let w = max(clog(s), 1); //we still need 1 bit to store "0"

        let mut rp = IntVec::new(w, n_bblocks); 
        let mut counted_bits = 0;
        let mut count = 0;
        for i in 0..(n_bblocks - 1){
            count += bv.get_int(i*b, b).count_ones(); // always count b bits.
            counted_bits += b;
            if counted_bits % s == 0 { count = 0 };
            rp.set_int(i + 1, count);
        }
        rp
    }

    pub fn print_repr(&self) {
        println!("n: {}, s: {}, b: {}", self.bv.len(), self.s, self.b);
        self.bv.print_bits();
        println!("{:?}", self.rs.to_vec());
        println!("{:?}", self.rb.to_vec());
    }

    pub fn rank(&self, i: usize) -> usize {
        assert!(i < self.len());
        let s_i = i / self.s;
        let r_s = self.rs.get_int(s_i);

        let b_i = i / self.b;
        let r_b = self.rb.get_int(b_i);

        let p_i = b_i * self.b;
        let width = (i % self.b) + 1;
        let w = self.bv.get_int(p_i, width);
        let r_p = w.count_ones();

        (r_s + r_b + r_p) as usize
    }

    pub fn len(&self) -> usize{
        self.bv.len()
    }

    pub fn size_of(&self) -> usize{
        let mut size = std::mem::size_of::<Self>();
        size += self.bv.size_of();
        size += self.rs.size_of();
        size += self.rb.size_of();
        size
    }

    pub fn overhead(&self) -> usize{
        self.size_of() * 8
    }
}

fn clog(x: usize) -> usize {
    let mut v = 0_usize;
    while x - 1 >> v != 0 {
        v += 1;
    }
    v
}

fn flog(x: usize) -> usize {
    let mut v = 0_usize;
    while x >> v != 0{
        v += 1;
    }
    v - 1
}

pub fn cdiv(a: usize, b: usize) -> usize {
    if a % b == 0 {
        a / b
    } else {
        (a / b) + 1
    }
} 
fn cdiv_2(x: usize) -> usize {
    (x >> 1) + (x & 1_usize)
}

fn fdiv_2(x: usize) -> usize {
    x >> 1
}

// fn sqr


#[cfg(test)]
mod tests {
    use crate::rank_select::*;
    #[test]
    fn rank_easy(){
        let bv = BitVec::from_bytes(&vec![0b10010111,0b01001010]);
        let rs = RankSupport::new(&bv);
        let rank = [1,1,1,2, 2,3,4,5, 5,6,6,6, 7,7,8,8];
        for i in 0..bv.len(){
            assert_eq!(rs.rank(i), rank[i]);
        }
    }

    #[test]
    fn rank_one(){
        let reps = 100;
        let bytes = &vec![!0u8; reps];
        let pad = 7_usize;
        let bv = BitVec::from_padded_bytes(bytes, pad);
        let n_bits = reps * 8 - pad;
        assert_eq!(bv.len(), n_bits);
        let rs = RankSupport::new(&bv);
        for i in 0..n_bits{
            assert_eq!(rs.rank(i), i+1);
        }
    }

    #[test]
    fn rank_evens(){
        let reps = 2;
        let bytes = &vec![!0b10101010; reps];
        let pad = 7_usize;
        let bv = BitVec::from_padded_bytes(bytes, pad);
        let n_bits = reps * 8 - pad;
        assert_eq!(bv.len(), n_bits);
        let rs = RankSupport::new(&bv);
        for i in 0..n_bits{
            assert_eq!(rs.rank(i), (i+1) / 2);
        }
    }

    #[test]
    fn rank_degenerate(){
        let bytes = vec![0b11000000];
        let bv = BitVec::from_padded_bytes(&bytes, 7);
        let rs = RankSupport::new(&bv);
        assert_eq!(rs.rank(0), 1);

        let bv = BitVec::from_padded_bytes(&bytes, 6);
        let rs = RankSupport::new(&bv);
        assert_eq!(rs.rank(0), 1);
        assert_eq!(rs.rank(1), 2);
    }

    #[test]
    fn rank_odds(){
        let reps = 2;
        let bytes = &vec![!0b01010101; reps];
        let pad = 7_usize;
        let bv = BitVec::from_padded_bytes(bytes, pad);
        let n_bits = reps * 8 - pad;
        assert_eq!(bv.len(), n_bits);
        let rs = RankSupport::new(&bv);
        for i in 0..n_bits{
            assert_eq!(rs.rank(i), (i / 2) + 1);
        }
    }

    #[test]
    fn test_clog(){
        assert_eq!(clog(1), 0);
        assert_eq!(clog(2), 1);
        assert_eq!(clog(15), 4);
        assert_eq!(clog(16), 4)
    }

    #[test]
    fn test_flog(){
        assert_eq!(flog(1), 0);
        assert_eq!(flog(2), 1);
        assert_eq!(flog(15), 3);
        assert_eq!(flog(16), 4);
        assert_eq!(flog(17), 4)

    }

    #[test]
    fn test_cdiv_2(){
        assert_eq!(cdiv_2(1), 1);
        assert_eq!(cdiv_2(10), 5);
        assert_eq!(cdiv_2(9), 5)
    }
}