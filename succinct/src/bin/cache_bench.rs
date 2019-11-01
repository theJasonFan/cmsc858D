use rand;
use rand::Rng;

use succinct::bv::BitVec;
use succinct::rank_select::RankSupport;
use succinct::rank_select::cdiv;

use std::env;
use std::time::Instant;

struct V {
    pub v: Vec<u32>
}


fn rand_indices(s: usize, repeats: usize) -> Vec<usize>{
    let mut rng = rand::thread_rng();
    (0..repeats).map(|_x| rng.gen::<usize>() % s).collect()
}

fn new(u: usize) -> V{
    V {
        v: vec![0u32; u]
    }
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
        let v = vec![0u8; *s];
        let t = Instant::now();
        let is = rand_indices(*s, repeats);
        for i in is.iter() {
            v.get(*i);
        }
        let elapsed = t.elapsed().as_nanos() as f32 / samples as f32;
        println!("{}\t{}\t{}", s*8, elapsed, s*8);
    }
}
