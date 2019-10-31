pub mod bv;
pub mod rank_select;

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

    let rs = rank_select::RankSupport::from_bytes(&vec![0b10010111,0b01001010]);
    rs.print_repr();

    for i in 0..rs.len() {print!("{}", rs.rank(i))}; println!();

    let rs = rank_select::RankSupport::from_bytes(&vec![0b10010111,0b01001010,0b01001010, 0b01001010]);
    rs.print_repr()
}