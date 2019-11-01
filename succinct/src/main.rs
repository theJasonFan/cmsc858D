// pub mod bv;
// pub mod rank_select;

use succinct::bv::BitVec;
use succinct::rank_select::RankSupport;

fn main() {
    println!("Hello, world!");
    // let mut v = bv::BitVec::new(128);
    // v.set_int(64-9, 7, 9);
    // //println!("{:?}", v.size_of());
    // println!("{:?}", bv::BitVec::new(32).size_of());
    // println!("{:?}",  bv::BitVec::new(32));
    // println!("{:?}", bv::IntVec::new(32,3).size_of());
    // println!("{:?}", bv::IntVec::new(32, 1));

    // rank_select::rs();

    // let bytes = vec![0b10100011,0b11000010];
    // let bv = bv::BitVec::from_bytes(&bytes);
    // bv.print_bits();

    // let iv = bv::IntVec::from_vec(&vec![1,2,3,0], 2);
    // println!("{:?}", iv.to_vec());
    // println!("{:?}", iv.size_of())

    // let reps = 2;
    // let bytes = &vec![!0b10101010; reps];
    // let pad = 7_usize;
    // let bv = BitVec::from_padded_bytes(bytes, pad);
    // let n_bits = reps * 8 - pad;
    // assert_eq!(bv.len(), n_bits);
    // let rs = RankSupport::new(&bv);
    // rs.print_repr();
    // bv.print_bits();
    // for i in 0..n_bits{
    //     assert_eq!(rs.rank(i), (i+1) / 2);
    // }

    let bytes = vec![0b10100011];
    let bv = BitVec::from_padded_bytes(&bytes, 6);
    bv.print_bits();
    let rs = RankSupport::new(bv);
    rs.print_repr();
}