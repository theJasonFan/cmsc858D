#[derive(Debug)]
pub struct BitVec {
    n: usize,
    blocks: Vec<u32>,
    n_blocks: usize,
}

impl BitVec {
    pub fn new(n: usize) -> Self {
        let n_blocks: usize;
        if n % 32_usize == 0 {
            n_blocks = n / 32_usize;
        } else {
            n_blocks = n / 32_usize + 1;
        }

		Self {
            n: n,
            blocks: vec![0; n_blocks],
            n_blocks: n_blocks,
		}
	}
    
    pub fn get(&self, i: usize) -> bool {
        self.get_int(i, 1) == 1     
    }

    pub fn set(&mut self, i: usize, v: bool) {
        if v {
            self.set_int(i, 1, 1);
        } else {
            self.set_int(i, 0, 1);
        }
    }

    pub fn get_int(&self, i: usize, w: usize) -> u32 {
        // within
        let b_i = i / 32_usize;

        let lo = i % 32_usize;
        let hi = (64_usize - lo - w) % 32_usize;
        
        //let v = self.blocks[i / 32];
        let mut v = 0;
        if lo + w <= 32 {
            let mut block = self.blocks[b_i];
            let mask = Self::get_mask(lo, w);
            block &= mask; // mask the other bits... 
            v = block >> hi; // shift over
        } else {
            let lblock = self.blocks[b_i] as u64;
            let rblock = self.blocks[b_i + 1] as u64;
            let mut block = rblock | (lblock << 32);
            block <<= lo;
            block >>= lo;
            block >>= hi;
            v = block as u32;
        }
        v
    }

    pub fn set_int(&mut self, i: usize, v: u32, w: usize) {
        Self::val_fits(v, w);

        let b_i = i / 32_usize;

        let lo = i % 32_usize;
        let hi = (64_usize - lo - w) % 32_usize;

        if lo + w <= 32 {
            let mut block = self.blocks[b_i];
            let mask = Self::get_mask(lo, w);
            block &= !mask;
            block |= v << hi; // shift and or bits into place
            self.blocks[b_i] = block; // insert

        } else {
            let mut lblock = self.blocks[b_i];
            let lmask = Self::get_mask(lo, 32 - lo);
            lblock &= !lmask;
            lblock |= v >> (32  - hi);
            self.blocks[b_i] = lblock;

            let mut rblock = self.blocks[b_i + 1];
            let rmask = Self::get_mask(0, 32 - hi);
            rblock &= !rmask; 
            rblock |= v << hi;
            self.blocks[b_i + 1] = rblock;
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    fn val_fits(v: u32, word_size: usize) -> bool {
        let mask = Self::get_mask(32 - word_size, word_size);
        !mask & v == 0u32
    }

    fn get_mask(i: usize, repeats:usize) ->u32{
        assert!(repeats <= 32);
        assert!((i + repeats) <= 32);
        if repeats == 0 {
            0u32
        } else if repeats == 32 {
            !0u32
        } else {
            let mut mask = !0u32;
            mask <<= 32 - repeats;
            mask >>= i;
            mask
        }
    }

    pub fn size_of(&self) -> usize {
        std::mem::size_of::<Self>() + std::mem::size_of_val::<[u32]>(&*self.blocks)
    }

    pub fn from_padded_bytes(bytes: &Vec<u8>, pad: usize) -> Self {
        assert!(pad <= 8);
        let n_bytes = bytes.len();
        let mut bv = Self::new(n_bytes * 8 - pad);
        let last = n_bytes - 1;
        for i in 0..last {
            bv.set_int(i*8, bytes[i] as u32, 8);
        }
        bv.set_int(last * 8, bytes[last] as u32 >> pad, 8 - pad);
        bv
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Self {
        Self::from_padded_bytes(bytes, 0)
    }

    pub fn print_bits(&self){
        for i in 0..self.len(){
            print!("{}", self.get(i) as u8);
        }
        print!("\n");
    }
}

// impl Copy for BitVec {
//     fn 
// }

#[derive(Debug)]
pub struct IntVec {
    word_size: usize,
    bv: BitVec,
    n: usize,
}

impl IntVec {
    pub fn new(w: usize, n: usize) -> Self {
        assert!(w > 0);
		Self {
            word_size: w,
            bv: BitVec::new(w * n),
            n: n
		}
	}

    pub fn get_int(&self, i: usize) -> u32 {
        assert!(i < self.len());
        self.bv.get_int(i * self.word_size, self.word_size)
    }

    pub fn set_int(&mut self, i: usize, v: u32) {
        assert!(i < self.len());
        self.bv.set_int(i * self.word_size, v, self.word_size)
    }

    pub fn len(&self) -> usize{
        self.n
    }

    pub fn size_of(&self) -> usize {
        std::mem::size_of::<Self>() + self.bv.size_of()
    }

    pub fn from_vec(elems: &Vec<u32>, w: usize) -> Self {
        let mut iv = Self::new(w, elems.len());
        for i in 0..iv.len() {
            iv.set_int(i, elems[i]);
        }
        iv
    }
    pub fn to_vec(&self) -> Vec<u32> {
        let mut v = vec![0; self.len()];
        for i in 0..self.len(){
            v[i] = self.get_int(i);
        }
        v
    }
}

#[cfg(test)]
mod tests {
    // // Note this useful idiom: importing names from outer (for mod tests) scope.
    use crate::bv::*;

    #[test]
    fn mask() {
        assert_eq!(BitVec::get_mask(31, 0), 0);
        assert_eq!(BitVec::get_mask(31, 1), 1);
        assert_eq!(BitVec::get_mask(29, 2), 6);

    }

    #[test]
    fn check_val() {
        assert!(BitVec::val_fits(7, 3));
        assert!(!(BitVec::val_fits(8, 3)));
    }

    #[test]
    fn set_easy() {
        let mut v = BitVec::new(32);
        v.set_int(0, 99, 32);
        assert_eq!(v.blocks[0], 99);

        let mut v = BitVec::new(64);
        v.set_int(64-9, 7, 9);
        assert_eq!(v.blocks[1], 7);

        let mut v = BitVec::new(128);
        v.set_int(128 - 32, 107, 31);
        assert_eq!(v.blocks[3], 214);
    }

    #[test]
    fn get_easy() {
        let mut v = BitVec::new(32);
        v.set_int(0, 99, 32);
        assert_eq!(v.get_int(0, 32), 99);

        let mut v = BitVec::new(64);
        v.set_int(64-9, 7, 9);
        assert_eq!(v.get_int(64-9, 9), 7);

        let mut v = BitVec::new(128);
        v.set_int(96, 107, 31);
        assert_eq!(v.get_int(96, 31), 107);
    }

    #[test]
    fn set_boundary() {
        // | 7, 7, 7, 7, [4 | 3], 7, 7, 7, 7, (1)|
        let mut v = BitVec::new(127);
        v.set_int(61, 31, 5);
        assert_eq!(v.blocks[1], 7);
        assert_eq!(v.blocks[2], 3 << 30);
    }

    #[test]
    fn get_boundary() {
        //insert 1100011
        //get 10001
        let mut v = BitVec::new(127);
        v.set_int(60, 0b1100011 as u32 , 7);
        assert_eq!(v.get_int(61, 5), 17);
    }

    #[test]
    fn fuzz_get_set(){
        let mut v = IntVec::new(7, 100);
        for i in 0..100 {
            v.set_int(i, (i % 128) as u32);
        }
        for i in 0..100 {
            assert_eq!(v.get_int(i), (i % 128) as u32);
        }
    }

}
