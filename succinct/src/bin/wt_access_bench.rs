use succinct::wt::WT;

use std::env;
use std::time::Instant;
use std::iter;
use rand::{Rng, thread_rng};
use std::cmp::{min};

use rand::distributions::Alphanumeric;

fn main() {
    /*
    USAGE:
        ./wt_n_bench [REPEATS] [SAMPLES] [MAX_EXP]
    
    Times ACCESS operation for wavelet tree with string lengths (0, 2^MAX_EXP] at SAMPLES even intervals
    We use random ASCII strings here, when the string length is large, \sigma == 128 is almost certain.
    We use this test as a proxy for fixing charset size.

    Outputs to stdout:
        3 lines corresponding to args of the run
        SAMPLES lines with format <size>\t<time>\t<overhead> where:
            <size> is the size of bitvector in bits
            <time> is the average time of the operation in nanoseconds
            <overhead> is the size of the RankSupport datastructure in bits.
    */

    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 4);
    let repeats: usize = args[1].parse().unwrap();
    let samples: usize = args[2].parse().unwrap();
    let power: usize = args[3].parse().unwrap();
    let max = 2_usize.pow(power as u32);

    println!("Repeats: {}", repeats);
    println!("Samples: {}", samples);
    println!("Max_exp Chars: {}", power);

    let mut rng = thread_rng();
    let incr = max / samples;
    let sizes: Vec<usize> = (0..samples).map(|x| (x + 1) * incr).collect();

    for s in sizes.iter() {
        let chars: String = iter::repeat(())
                            .map(|()| rng.sample(Alphanumeric))
                            .take(*s)
                            .collect();
        let wt = WT::new(&chars);
        let t = Instant::now();
        let c = chars.chars().nth(0).unwrap();
        for i in 0..repeats {
            wt.access(min(i, *s));
        }
        let elapsed = (t.elapsed().as_nanos() as f32) / (repeats as f32);
        let size = wt.size_of(); // in bytes
        println!("{}\t{}\t{}", s, elapsed, size);
    }
}