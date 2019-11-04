// pub mod bv;


use succinct::bv::BitVec;
use succinct::rank_select::RankSupport;

use succinct::wt::WTBuilder;
fn main() {
    let s = "tomorrow and tomorrow and tomorrow";
    let mut wtb = WTBuilder::new(&s);
    wtb.build();
    println!("new");
    wtb.print_repr();

    let wt = wtb.finish();
    // let ranks = [1,2,3,4];
    // for r in ranks.iter() {
    //     println!("{}, {}", 'a', wt.select('a', *r).unwrap());
    // }

    // println!("{}", wt.rank('d', 0));

    for (i, c) in s.chars().enumerate() {
        print!("{}", wt.access(i));
    }
    println!();

    // let bv = BitVec::from_bytes(&vec![0b01001010]);
    // let rs = RankSupport::new(bv);
    // assert_eq!(rs.select0(1), Some(0));
    // assert_eq!(rs.select1(1), Some(1));
    // assert_eq!(rs.select0(2), Some(2));
    // assert_eq!(rs.select0(3), Some(3));
    // assert_eq!(rs.select1(2), Some(4));
    // assert_eq!(rs.select0(4), Some(5));
    // assert_eq!(rs.select1(3), Some(6));
    // assert_eq!(rs.select0(5), Some(7));


    // let bv = BitVec::from_bytes(&vec![!0u8]);
    // let rs = RankSupport::new(bv);
    // assert_eq!(rs.select0(1), None);

}