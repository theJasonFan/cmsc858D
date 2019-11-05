use rand;
use rand::Rng;

use succinct::bv::BitVec;

use std::env;
use std::time::Instant;

fn main() {
    /*
    USAGE:
        ./bv_bench [REPEATS] [SAMPLES] [MAX_EXP]
    
    Times bit-vector get for bit-vectors of size (0, 2^MAX_EXP] at SAMPLES even intervals

    Outputs to stdout:
        3 lines corresponding to args of the run
        SAMPLES lines with format <size>\t<time>\t<overhead> where:
            <size> is the size of bitvector in bits
            <time> is the average time of the operation in nanoseconds
            <overhead> is the size of the bitvec datastructure in bits.
    */

    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 4);
    let repeats: usize = args[1].parse().unwrap();
    let samples: usize = args[2].parse().unwrap();
    let power: usize = args[3].parse().unwrap();
    let max = 2_usize.pow(power as u32);

    println!("Repeats: {}", repeats);
    println!("Samples: {}", samples);
    println!("Max_exp Bits: {}", power);

    let incr = max / samples;
    let sizes: Vec<usize> = (0..samples).map(|x| (x + 1) * incr).collect();

    for s in sizes.iter() {
        let bv = BitVec::new(*s);
        
        // Turns out cache accesses matter!
        let t = Instant::now();
        for i in 0..repeats {
            bv.get( (i + (i % 2) * (s / 2)) % s);
        }

        let elapsed = (t.elapsed().as_nanos() as f32) / (repeats as f32);
        let overhead = bv.size_of() * 8;
        println!("{}\t{}\t{}", s, elapsed, overhead);
    }
}