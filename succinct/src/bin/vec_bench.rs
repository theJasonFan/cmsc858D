use rand;
use rand::Rng;

use succinct::bv::BitVec;
use succinct::rank_select::RankSupport;

use std::env;
use std::time::Instant;

fn rand_indices(s: usize, repeats: usize) -> Vec<usize>{
    let mut rng = rand::thread_rng();
    (0..repeats).map(|_x| rng.gen::<usize>() % s).collect()
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 5);
    let repeats: usize = args[1].parse().unwrap();
    let exps: usize = args[2].parse().unwrap();
    let max: usize = args[4].parse().unwrap();
    let min: usize = args[3].parse().unwrap();
    println!("Repeats: {}", repeats);
    println!("Samples: {}", exps);
    println!("Min Bits: {}", min);
    println!("Max Bits: {}", max);

    let incr = (max - min) / exps;
    let sizes: Vec<usize> = (1..(exps + 1)).map(|x| min + (x * incr)).collect();

    for s in sizes.iter() {
        let bytes = vec![0; *s];
        let bv = BitVec::from_bytes(&bytes);
        let is = rand_indices(*s, exps);
        let t = Instant::now();

        for i in is.iter() {
            let mut b = bv.get(*i);
        }
        let elapsed = t.elapsed().as_nanos() as f32 / exps as f32;
        println!("{}\t{}\t{}", s, elapsed, s);
    }
}