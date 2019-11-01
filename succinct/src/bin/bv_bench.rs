use rand;
use rand::Rng;

use succinct::bv::BitVec;
use succinct::rank_select::RankSupport;
use succinct::rank_select::cdiv;

use std::env;
use std::time::Instant;

fn rand_indices(s: usize, repeats: usize) -> Vec<usize>{
    let mut rng = rand::thread_rng();
    (0..repeats).map(|_x| rng.gen::<usize>() % s).collect()
}

fn main() {
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
        let is = rand_indices(*s, repeats);
        assert_eq!(is.len(), repeats);

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