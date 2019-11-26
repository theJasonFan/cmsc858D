use succinct::wt::WT;

use std::env;
use std::time::Instant;
use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

fn main() {
    /*
    USAGE:
        ./wt_sigma_bench [REPEATS] [STRING_LEN]
    
    Times RANK operation for wavelet tree for given string length with varying alphabet size [2, 128]

    Outputs to stdout:
        2 lines corresponding to args of the run
        SAMPLES lines with format <size>\t<time>\t<overhead> where:
            <size> is the size of alphabet in bits
            <time> is the average time of the operation in nanoseconds
            <overhead> is the size of the WT datastructure in bytes.
    */

    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3);
    let repeats: usize = args[1].parse().unwrap();
    let string_len: usize = args[2].parse().unwrap();
    let max_sigma = 128; //ascii
    let min_sigma = 2;

    let mut rng = thread_rng();
    println!("repeats {}", repeats);
    println!("string length {}", string_len);

    for s in min_sigma..max_sigma + 1 {
        let chars: String = iter::repeat(())
                            .map(|()| ((rng.sample(Alphanumeric) as u8 % s as u8) as char))
                            .take(string_len)
                            .collect();
        let wt = WT::new(&chars);
        let t = Instant::now();
        let c = chars.chars().nth(0).unwrap();
        for i in 0..repeats {
            wt.rank(c, (i + (i % 2) * (string_len / 2)) % string_len);
        }
        let elapsed = (t.elapsed().as_nanos() as f32) / (repeats as f32);
        let size = wt.size_of(); // in bytes
        println!("{}\t{}\t{}", s, elapsed, size);
    }
}