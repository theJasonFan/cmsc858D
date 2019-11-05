use super::bv::{IntVec, BitVec};
use super::rank_select::RankSupport;
use super::math::{clog};
use serde::{Serialize, Deserialize};
// use std::str;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharTable {
    // minimal bit representation of ascii chars
    rs: RankSupport,
    width: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WT {
    n: usize,
    bv: Vec<RankSupport>,
    char_table: CharTable,
}

#[derive(Debug)]
pub struct WTBuilder<'a> {
    s: &'a str,
    char_table: CharTable,
    l: usize, 
    n_chars: usize,
    n: usize,
    bv: Vec<BitVec>,
    hist: IntVec, 
    spos: IntVec,
}

impl<'a> WTBuilder<'a> {
    // Wavelet Tree builder class for Fischer, Kurpicz, and Noble. 
    // "Simple, Fast and Lightweight Parallel Wavelet Tree Construction"
    // Here, we use the pcWT (sequential) algorithm

    pub fn new(s: &'a str)  -> Self {
        assert!(s.is_ascii());
        let n = s.len(); //todo
        let n_chars = count_chars(s);
        let l = clog(n_chars); //Todo

        let ct = CharTable::new(s);

        // n log(sigma) for the bitvectors...
        let bv = vec![BitVec::new(n); l];
        
        // sigma log(n) bits for HIST...
        let hist = IntVec::new(clog(n), 2_usize.pow(l as u32) as usize); //oversize if log is not round

        // sigma log(n) bits for starting positions of blocks
        let spos = IntVec::new(clog(n), 2_usize.pow(l as u32) as usize); //oversize if log is not round
        Self {
            s: s,
            n: n,
            char_table: ct,
            l: l,
            n_chars: n_chars,
            bv: bv,
            hist: hist,
            spos: spos,
        }
    }

    fn init_hist(&mut self) {
        // Base histogram
        for c in self.s.chars() {
            let c_i = self.char_table.i(c);
            self.hist.set_int(c_i, self.hist.get_int(c_i) + 1);
        }
    }

    fn init_bv(&mut self) {
        for (i, c) in self.s.chars().enumerate() {
            self.bv[0].set(i, self.char_table.get_bit(0, c))
        }
    }

    pub fn build(&mut self) -> &Self {
        // Build the wavelent tree bit vectors.

        if self.n_chars == 1 { return self }

        self.init_hist();
        self.init_bv();
        // for [l-1 to 1]
        for i in 0..(self.l-1) {
            let li = self.l - 1 - i;

            // Histogram of the prefixes
            for i in 0.. 2_usize.pow(li as u32) {
                let hist2i_plus1 = self.hist.get_int(2*i + 1);
                let hist2i = self.hist.get_int(2*i);
                self.hist.set_int(i, hist2i + hist2i_plus1);
            }

            // Update SPos
            self.spos.set_int(0, 0); // NOT SURE WHY THIS IS NOT IN THE TEX'd ALG
            for i in 1.. 2_usize.pow(li as u32) {
                let spos_i_minus1 = self.spos.get_int(i-1);
                let hist_i_minus1 = self.hist.get_int(i-1);
                if spos_i_minus1 + hist_i_minus1 < self.n as u32 {
                    // avoid edge case where we make spos length of the array.
                    // NOT SURE WHY THIS IS NOT IN THE TEX'd ALG
                    self.spos.set_int(i, spos_i_minus1 + hist_i_minus1);
                }
            }

            // Insert chars in into BitVec at level l_i
            for c in self.s.chars() {
                let li_prefix = self.char_table.get_prefix(li, c);
                let pos = self.spos.get_int(li_prefix);

                if pos + 1 < self.n as u32 {
                    // avoid edge case where we make pos + 1 length of the array.
                    // NOT SURE WHY THIS IS NOT IN THE TEX'd ALG
                    self.spos.set_int(li_prefix, pos + 1); //increase the position by 1
                }
                self.bv[li].set(pos as usize, self.char_table.get_bit(li, c));
            }
        }
        self
    }

    pub fn finish(&self) -> WT {    
        // Complete the construction. Create rank supported bit vectors.
        let mut bv = vec![];
        for bv_i in self.bv.iter() {
            bv.push(RankSupport::new(bv_i.clone()));
        }
        WT {
            n: self.n,
            bv: bv,
            char_table: self.char_table.clone(),
        }
    }

    pub fn print_repr(&self) {
        println!("{}", self.s);
        for bv in self.bv.iter() {
            bv.print_bits();
        }
    }
}

impl WT {
    pub fn new(s: &str)  -> Self {
        assert!(s.is_ascii());
        WTBuilder::new(s).build().finish()
    }

    pub fn access(&self, i: usize) -> char {
        assert!(i < self.n);
        if self.bv.len() == 0 { return self.char_table.get_char(0)}
        let mut l = 0;
        let mut r = self.n;
        let last_l = self.char_table.width;
        let mut curr_rank = i + 1;

        let mut char_i = 0_usize;
        for i in 0..last_l {
            // look at the bit I care about (The rank in the interval in this level)
            let curr_bit = self.bv[i].get(l + curr_rank - 1);

            // Shift and set lowest order bit.
            if curr_bit {
                char_i = (char_i << 1) + 1
            } else {
                char_i = char_i << 1
            }

            // Update the rank to look at in next level
            curr_rank = self.bv[i].rel_rank(curr_bit, l, curr_rank - 1);

            // Figure out the offset for the next level
            if curr_bit { // go right
                // count the zeros to get to the offset
                l += self.bv[i].rel_rank(false, l,  r - l - 1);
            } else { // go left
                r -= self.bv[i].rel_rank(true, l,  r - l - 1);
            }
        }
        self.char_table.get_char(char_i)
    }

    pub fn rank(&self, c: char, i: usize) -> usize {
        if self.bv.len() == 0 { return i + 1 }
        let mut l = 0;
        let mut r = self.n;
        let last_l = self.char_table.width;
        let mut curr_rank = i + 1;

        for i in 0..last_l {
            // Get next highest order bit to figure out traversal
            let curr_bit = self.char_table.get_bit(i, c);

            // Update the rank to look at in next level
            curr_rank = self.bv[i].rel_rank(curr_bit, l, curr_rank - 1);

            // if the rank is 0 in the chunk then we can just return 0.
            if curr_rank == 0 { return curr_rank};

            // Figure out the offset for the next level
            if curr_bit { // go right
                // count the zeros to get to the offset
                l += self.bv[i].rel_rank(false, l,  r - l - 1);
            } else { // go left
                r -= self.bv[i].rel_rank(true, l,  r - l - 1);
            }
        }
        curr_rank
    }

    pub fn select(&self, c: char, rank: usize) -> Option<usize> {
        if self.bv.len() == 0 { return Some (rank - 1) }
        let mut l = 0;
        let mut r = self.n;
        let last_l = self.char_table.width;
        let mut curr_bit: bool;
        let mut stack = vec![];

        for i in 0..last_l {
            // Get next highest order bit to figure out traversal
            curr_bit = self.char_table.get_bit(i, c);

            // Push the offset and the current bit onto the stack
            stack.push((l,curr_bit));

            // Figure out the offset for the next level
            if curr_bit { // go right
                // count the zeros to get to the offset
                l += self.bv[i].rel_rank(false, l,  r - l - 1);
            } else { // go left
                r -= self.bv[i].rel_rank(true, l,  r - l - 1);
            }
        }

        // Go from bottom to top, popping the stack..
        
        // Start with the index in the bottom interval as rank-1
        let mut curr_index = rank - 1;
        for i in 0..last_l {

            // Get offset and corresponding bit in the next level up
            let  lb = stack.pop().unwrap();

            // The next index to look at is the select of rank=(curr_index + 1)
            let option = self.bv[last_l - 1 - i].rel_select(lb.1, lb.0, curr_index + 1);
            if option == None {return None}
            curr_index = option.unwrap();
        }

        Some(curr_index)
    } 

    pub fn size_of(&self) -> usize {
        // Size of struct in bytes
        let mut size = std::mem::size_of::<Self>();
        size += self.bv.len() * self.bv[0].size_of();
        size += self.char_table.size_of();
        size
    }

    pub fn n_chars(&self) -> usize {
        self.char_table.n_chars()
    }

    pub fn len(&self) -> usize {
        self.n
    }
}


impl CharTable {
    pub fn new(s: &str) -> Self {
        assert!(s.is_ascii());

        let mut bv = BitVec::new(128);
        for c in s.chars() {
            bv.set(c as usize, true);
        }
        Self {
            width: clog(count_chars(s)),
            rs: RankSupport::new(bv)
        }
    }

    pub fn i(&self, c: char) -> usize {
        // Embedding is the rank of the char of the bitvector of possible asciis
        assert!(self.in_charset(c));
        let c_i = c as usize;
        self.rs.rank1(c_i) - 1
    }

    pub fn get_bit(&self, i: usize, c: char) -> bool {
        assert!(i < self.width);
        assert!(self.in_charset(c));
        let bits = self.i(c);
        let mut mask = 1usize;
        mask <<= self.width - 1; // to most significnt bit
        mask >>=i;               // to the index
        mask & bits != 0
    }

    // used for indexing so we return usize instead of u32
    pub fn get_prefix(&self, l: usize, c: char) -> usize {
        assert!(l > 0);
        assert!(l <= self.width);
        assert!(self.in_charset(c));
        self.i(c) >> (self.width - l)
    }

    pub fn get_char(&self, char_i: usize) -> char {
        self.rs.select1(char_i + 1).unwrap() as u8 as char
    }

    pub fn in_charset(&self, c: char) -> bool {
        let c_i = c as usize;
        c.is_ascii() && self.rs.get(c_i)
    }

    pub fn size_of(&self) -> usize {
        let mut size = std::mem::size_of::<Self>();
        size += self.rs.size_of();
        size
    }

    pub fn n_chars(&self) -> usize {
        // Size of encoded alphabet
        self.rs.rank1(127)
    }
}

pub fn count_chars(s: &str) -> usize {
    assert!(s.is_ascii());
    let mut table = [0; 128];
    for c in s.chars() {
        table[c as usize] = 1;
    }
    table.iter().sum()
}

#[cfg(test)]
mod wt_tests {
    use crate::wt::*;

    #[test]
    fn access() {
        let s = "abracadabra";
        let wt = WT::new(&s);
        for (i, c) in s.chars().enumerate() {
            assert_eq!(wt.access(i), c);
        }

        let s = "yabadabadoy";
        let wt = WT::new(&s);
        for (i, c) in s.chars().enumerate() {
            assert_eq!(wt.access(i), c);
        }

        let s = "tomorrow and tomorrow and tomorrow";
        let wt = WT::new(&s);
        for (i, c) in s.chars().enumerate() {
            assert_eq!(wt.access(i), c);
        }

    }

    #[test]
    fn select() {
        let s = "abracadabra";
        let wt = WT::new(&s);
        for (i, c) in s.chars().enumerate() {
            println!("{}, {}", i, c);
            assert_eq!(i, wt.select(c, wt.rank(c, i)).unwrap());
        }

        let s = "yabadabadoo";
        let wt = WT::new(&s);
        for (i, c) in s.chars().enumerate() {
            println!("{}, {}", i, c);
            assert_eq!(i, wt.select(c, wt.rank(c, i)).unwrap());
        }

        let s = "aaaaaaa";
        let wt = WT::new(&s);
        for (i, c) in s.chars().enumerate() {
            println!("{}, {}", i, c);
            assert_eq!(i, wt.select(c, wt.rank(c, i)).unwrap());
        }
    }

    #[test]
    fn rank() {
        let s = "abracadabra";
        let wt = WT::new(&s);
        let ranks = [1,1,1,2,1,3,1,4,2,2,5];
        for (i, c) in s.chars().enumerate() {
            assert_eq!(wt.rank(c, i), ranks[i]);
        }

        let s = "yabadabadoy";
        let wt = WT::new(&s);
        let ranks = [1,1,1,2,1,3,2,4,2,1,2];
        for (i, c) in s.chars().enumerate() {
            assert_eq!(wt.rank(c, i), ranks[i]);
        }

        let s = "aaaaa";
        let ranks = [1,2,3,4,5];
        let wt = WT::new(&s);
        for (i, c) in s.chars().enumerate() {
            assert_eq!(wt.rank(c, i), ranks[i]);
        }
    }

    #[test]
    fn rank_hard() {
        let s = "yabadabadooy";
        let wt = WT::new(&s);
        let ranks = [0,1,1,2,2,3,3,4,4,4,4,4];
        for (i, _c) in s.chars().enumerate() {
            assert_eq!(wt.rank('a', i), ranks[i]);
        }

        let ranks = [0,0,1,1,1,1,2,2,2,2,2,2];
        for (i, _c) in s.chars().enumerate() {
            assert_eq!(wt.rank('b', i), ranks[i]);
        }

        let ranks = [0,0,0,0,1,1,1,1,2,2,2,2];
        for (i, _c) in s.chars().enumerate() {
            assert_eq!(wt.rank('d', i), ranks[i]);
        }

        let ranks = [0,0,0,0,0,0,0,0,0,1,2,2];
        for (i, _c) in s.chars().enumerate() {
            assert_eq!(wt.rank('o', i), ranks[i]);
        }

        let ranks = [1,1,1,1,1,1,1,1,1,1,1,2];
        for (i, _c) in s.chars().enumerate() {
            assert_eq!(wt.rank('y', i), ranks[i]);
        }
    }

    #[test]
    fn count_c() {
        let s = "0167154263";
        assert_eq!(count_chars(&s),8);
    }
}

#[cfg(test)]
mod wtbuilder_tests {
    use crate::wt::*;

    #[test]
    fn new() {
        let s = "0167154263";
        let n = 10_usize;
        let sigma = 8_usize;

        let wtb = WTBuilder::new(&s);

        assert_eq!(wtb.hist.len(), sigma);
        assert_eq!(wtb.hist.w_size(), clog(n));
        assert_eq!(wtb.spos.len(), sigma);
        assert_eq!(wtb.spos.w_size(), clog(n));
    }

    fn bv_same(v1: &[bool], v2: &[bool]) -> bool{
        // https://stackoverflow.com/questions/40767815/how-do-i-check-whether-a-vector-is-equal-to-another-vector-that-contains-nan-and
        v1.iter()
          .zip(v2)
          .all(|(b1, b2)| b1 == b2)
    }

    #[test]
    fn build () {
        let s = "0167154263";
        let mut wtb = WTBuilder::new(&s);
        wtb.build();
        let bv0 = [false, false, true, true, false, true, true, false, true, false];
        assert!(bv_same(&bv0, &wtb.bv[0].to_vec()));

        let bv1 = [false, false, false, true, true, true, true, false, false, true];
        assert!(bv_same(&bv1, &wtb.bv[1].to_vec()));

        let bv2 = [false, true, true, false, true, true, false, false, true, false];
        assert!(bv_same(&bv2, &wtb.bv[2].to_vec()));

        let s = "01234";
        let mut wtb = WTBuilder::new(&s);
        assert_eq!(wtb.hist.len(), 8);
        wtb.build();
    }

    #[test]
    fn init_hist() {
        let s = "dccbbbaaaa";
        let mut wtb = WTBuilder::new(&s);
        wtb.init_hist();

        assert_eq!(wtb.hist.get_int(0), 4);
        assert_eq!(wtb.hist.get_int(1), 3);
        assert_eq!(wtb.hist.get_int(2), 2);
        assert_eq!(wtb.hist.get_int(3), 1);
    }

    #[test]
    fn init_bv() {
        let s = "0167154263";
        let mut wtb = WTBuilder::new(&s);
        wtb.init_bv();
        let bits = [false, false, true, true, false, true, true, false, true, false];
        let bv0 = &wtb.bv[0];
        for i in 0..bv0.len() {
            assert_eq!(bv0.get(i), bits[i]);
        }
    }

    fn get_char() {
        let ct = CharTable::new("0167154263");
        assert_eq!(ct.get_char(0), '0');
        assert_eq!(ct.get_char(1), '1');
        assert_eq!(ct.get_char(2), '2');
        assert_eq!(ct.get_char(3), '3');
        assert_eq!(ct.get_char(4), '4');
        assert_eq!(ct.get_char(5), '5');
        assert_eq!(ct.get_char(6), '6');
        assert_eq!(ct.get_char(7), '7');
    }

    #[test]
    fn char_table() {
        let ct = CharTable::new("0167154263");
        assert_eq!(ct.i('0'), 0);
        assert_eq!(ct.i('1'), 1);
        assert_eq!(ct.i('2'), 2);
        assert_eq!(ct.i('3'), 3);
        assert_eq!(ct.i('4'), 4);
        assert_eq!(ct.i('5'), 5);
        assert_eq!(ct.i('6'), 6);
        assert_eq!(ct.i('7'), 7);
        assert_eq!(ct.get_bit(0, '5'), true);
        assert_eq!(ct.get_bit(1, '5'), false);
        assert_eq!(ct.get_bit(2, '5'), true);

        assert_eq!(ct.get_prefix(3, '7'), 7);
        assert_eq!(ct.get_prefix(2, '7'), 3);
        assert_eq!(ct.get_prefix(1, '7'), 1);

        assert_eq!(ct.get_prefix(3, '0'), 0);
        assert_eq!(ct.get_prefix(2, '0'), 0);
        assert_eq!(ct.get_prefix(1, '0'), 0);

        assert_eq!(ct.get_prefix(3, '5'), 5);
        assert_eq!(ct.get_prefix(2, '5'), 2);
        assert_eq!(ct.get_prefix(1, '5'), 1);

        assert_eq!(ct.get_prefix(3, '6'), 6);
        assert_eq!(ct.get_prefix(2, '6'), 3);
        assert_eq!(ct.get_prefix(1, '6'), 1);

        let ct = CharTable::new("tcga");
        assert_eq!(ct.i('a'), 0);
        assert_eq!(ct.i('c'), 1);
        assert_eq!(ct.i('g'), 2);
        assert_eq!(ct.i('t'), 3);

        assert_eq!(ct.get_bit(0, 'a'), false);
        assert_eq!(ct.get_bit(1, 'a'), false);
        assert_eq!(ct.get_bit(0, 'c'), false);
        assert_eq!(ct.get_bit(1, 'c'), true);
        assert_eq!(ct.get_bit(0, 'g'), true);
        assert_eq!(ct.get_bit(1, 'g'), false);
        assert_eq!(ct.get_bit(0, 't'), true);
        assert_eq!(ct.get_bit(1, 't'), true);
    }

}