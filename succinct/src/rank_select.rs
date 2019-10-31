use super::bv::{IntVec, BitVec};
use std::cmp::min;

pub fn rs() {
    println!("Hello, world!");
    let mut v = BitVec::new(128);
    v.set_int(64-9, 7, 9);
    //println!("{:?}", v.size_of());
    println!("{:?}", BitVec::new(32).size_of());
    println!("{:?}",  BitVec::new(32));
    println!("{:?}", IntVec::new(32,3).size_of());
    println!("{:?}", IntVec::new(32, 1));
}

#[derive(Debug)]
pub struct RankSupport {
    // pub bv: &'a BitVec,
    pub bv: BitVec,
    pub s: usize, // Probably can be smaller uint
    pub b: usize, 
    pub rs: IntVec,
    pub rb: IntVec,
}

impl RankSupport {

    pub fn from_bytes(bytes: &Vec<u8>) -> Self{
       let bv = BitVec::from_bytes(bytes);
       Self::new(bv)
    }
    
    pub fn new (bv: BitVec) -> Self {
        let s = cdiv_2(clog(bv.len())*clog(bv.len()));
        let b = cdiv_2(clog(bv.len()));
        let s = (s / b) * b; // hack to make s divisible! no more book keeping...
        println!("n: {}, s: {}, b: {}", bv.len(), s, b);
        assert_eq!(s % b, 0);

        Self {
                bv: bv.clone(), //? is there a better way to do this?
                s: s,
                b: b,
                rs: Self::get_rs(&bv, s),
                rb: Self::get_rb(&bv, s,  b)
            }
    }

    pub fn  get_rs(bv: &BitVec, s: usize) -> IntVec{
        let n = bv.len();
        let n_blocks = cdiv(n, s);
        let mut rs = IntVec::new(clog(n), n_blocks);
        let mut count = 0;
        // First N blocks...
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

    pub fn get_rb(bv: &BitVec, s: usize, b:usize) -> IntVec {
        let n = bv.len();
        let n_bblocks = cdiv(n, b);
        let mut rp = IntVec::new(clog(s), n_bblocks);

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

    pub fn rank(&self, i: usize) -> u32 {
        let s_i = i / self.s;
        let r_s = self.rs.get_int(s_i);

        let b_i = i / self.b;
        let r_b = self.rb.get_int(b_i);

        let p_i = b_i * self.b;
        let width = (i % self.b) + 1;
        let w = self.bv.get_int(p_i, width);
        let r_p = w.count_ones();

        r_s + r_b + r_p
    }

    pub fn len(&self) -> usize{
        self.bv.len()
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

fn cdiv(a: usize, b: usize) -> usize {
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
        let rs = RankSupport::from_bytes(&vec![0b10010111,0b01001010]);
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