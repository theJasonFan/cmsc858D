use super::bv::{IntVec, BitVec};
use super::rank_select::RankSupport;
use super::math::{clog};
// use std::str;

#[derive(Debug, Clone)]
pub struct CharTable {
    // minimal bit representation of ascii chars
    rs: RankSupport,
    width: usize,
}

#[derive(Debug)]
pub struct WT {
    bv: Vec<RankSupport>,
    char_table: CharTable,
}

#[derive(Debug)]
pub struct WTBuilder<'a> {
    s: &'a str,
    char_table: CharTable,
    l: usize, 
    n_chars: usize,
    bv: Vec<BitVec>,
    hist: IntVec, 
    spos: IntVec,
}

impl<'a> WTBuilder<'a> {

    pub fn new(s: &'a str)  -> Self {
        assert!(s.is_ascii());
        let n = s.len(); //todo
        let n_chars = count_chars(s);
        let l = clog(n_chars); //Todo

        let ct = CharTable::new(s);

        // n log(sigma) for the bitvectors...
        let bv = vec![BitVec::new(n); l];
        
        //sigma log(n) bits for HIST...
        let hist = IntVec::new(clog(n), 2_usize.pow(l as u32) as usize); //oversize if log is not round

        // sigma log(n) bits for starting positions of blocks
        let spos = IntVec::new(clog(n), 2_usize.pow(l as u32) as usize); //oversize if log is not round

        Self {
            s: s,
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
        self.init_hist();
        self.init_bv();
        
        // for [l-1 to 1]
        for i in 0..(self.l-1) {
            let li = self.l - 1 - i;

            for i in 0.. 2_usize.pow(li as u32) {
                let hist2i_plus1 = self.hist.get_int(2*i + 1);
                let hist2i = self.hist.get_int(2*i);
                self.hist.set_int(i, hist2i + hist2i_plus1);
            }

            self.spos.set_int(0, 0); // NOT SURE WHY THIS IS NOT IN THE TEX'd ALG
            for i in 1.. 2_usize.pow(li as u32) {
                let spos_i_minus1 = self.spos.get_int(i-1);
                let hist_i_minus1 = self.hist.get_int(i-1);
                self.spos.set_int(i, spos_i_minus1 + hist_i_minus1);
            }

            for (i, c) in self.s.chars().enumerate() {
                let li_prefix = self.char_table.get_prefix(li, c);
                let pos = self.spos.get_int(li_prefix);
                self.spos.set_int(li_prefix, pos + 1); //increase the position by 1
                self.bv[li].set(pos as usize, self.char_table.get_bit(li, c));
            }
        }
        self
    }

    pub fn finish(&self) -> WT {    
        let mut bv = vec![];
        for bv_i in self.bv.iter() {
            bv.push(RankSupport::new(bv_i.clone()));
        }
        WT {
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
}


impl CharTable {
    pub fn new(s: &str) -> Self {
        assert!(s.is_ascii());

        let mut bv = BitVec::new(128); //ascii only
        for c in s.chars() {
            bv.set(c as usize, true);
        }
        Self {
            width: clog(count_chars(s)),
            rs: RankSupport::new(bv)
        }
    }

    pub fn i(&self, c: char) -> usize {
        assert!(self.in_charset(c));
        let c_i = c as usize;
        self.rs.rank1(c_i) - 1
    }

    pub fn get_bit(&self, i: usize, c: char) -> bool {
        assert!(i < self.width);
        assert!(self.in_charset(c));
        let mut bits = self.i(c);
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

    fn in_charset(&self, c: char) -> bool {
        let c_i = c as usize;
        c.is_ascii() && self.rs.get(c_i)
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
mod WT_tests {
    use crate::wt::*;

    #[test]
    fn new() {
        let s = "0167154263";
        let wt = WT::new(&s);
    }

    #[test]
    fn count_c() {
        let s = "0167154263";
        assert_eq!(count_chars(&s),8);
    }
}

#[cfg(test)]
mod WTBuilder_tests {
    use crate::wt::*;

    #[test]
    fn new() {
        let s = "0167154263";
        let n = 10_usize;
        let l = 3_usize;
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
        assert!(bv_same(&bv1, &wtb.bv[1].to_vec()));

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