// pub mod bv;
// pub mod rank_select;

use succinct::wt::WTBuilder;
fn main() {
    let s = "123456789";
    let mut wtb = WTBuilder::new(&s);
    wtb.build();
    wtb.print_repr();
}