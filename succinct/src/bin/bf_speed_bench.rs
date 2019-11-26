use succinct::bloom_filter::{BlockedBloomFilter, BloomFilter, MQ};
use std::hash::Hash;

use std::env;
use std::time::Instant;


fn main() {
    /*
    USAGE:
        ./bv_bench [REPEATS] [SAMPLES] [MAX_EXP]
    
    Times bit-vector get for bit-vectors of size (0, 2^MAX_EXP] at SAMPLES even intervals
    */
    
    //let args: Vec<String> = env::args().collect();

    let n_Ns = 2;
    let max_N = 5000000;
    let min_N = 1000;

    let n_fprs = 2;
    let min_fpr = 0.01;
    let max_fpr = 0.05;

    let incr_N = (max_N - min_N) / (n_Ns - 1);
    let ns: Vec<usize> = (0..n_Ns).map(|x| x * incr_N + min_N).collect();

    let incr_fpr = (max_fpr - min_fpr) / n_fprs as f32;
    let fprs: Vec<f32> = (0..n_fprs).map(|x| x as f32 * incr_fpr + min_fpr).collect();
    println!("{}\t{}\t{}\t{}\t{}","N", "FPR", "empirical_FPR",  "query_time", "frac_query_inserted");

    for n in ns.iter() {
        for fpr in fprs.iter() {
            // let mut bf = BloomFilter::with_fpr(*fpr, *n);
            // report_fpr_1(*n, *fpr, &mut bf);
            let mut bf = BloomFilter::with_fpr(*fpr, *n);
            report_fpr_3(*n, *fpr, &mut bf);

            // let mut bf = BlockedBloomFilter::with_fpr(*fpr, *n, 64_usize);
            // report_fpr_1(*n, *fpr, &mut bf);

            let mut bf = BlockedBloomFilter::with_fpr(*fpr, *n, 64_usize);
            report_fpr_3(*n, *fpr, &mut bf);
            // report_fpr_2(*n, *fpr);
        }
    }
}

fn timed_query<H: Hash, T: MQ>(x: &H, amq: &T) -> (bool, usize) {
    let t = Instant::now();
    let isin = amq.query(x);
    let elapsed = t.elapsed().as_nanos() as usize;
    (isin, elapsed)
}

fn fpr_and_query_time<T: MQ>(N:usize, min_not_inserted: usize, amq: &T) -> (f32, f32) {
    let mut n_true_neg = 0;
    let mut n_false_pos = 0;
    let mut total_elapsed = 0;
    for i in 0..N {
        let (isin, elapsed) = timed_query(&i, amq);

        if i >= min_not_inserted {
            if isin {
                n_false_pos += 1;
            }
            n_true_neg += 1;
        }
        total_elapsed += elapsed;
    }
    let fpr = n_false_pos as f32 / n_true_neg as f32;
    let ns_per_query = total_elapsed as f32 / N as f32;
    (fpr, ns_per_query)
}

fn report_fpr_1<T: MQ>(n: usize, fpr: f32, amq: &mut T) {
    let min_not_inserted = n / 2;

    for i in 0..(n/2) {
        amq.insert(&i);
    }
    let (e_fpr, query_time) = fpr_and_query_time(n, min_not_inserted, amq);
    println!("{}\t{}\t{}\t{}\t{}",n, fpr, e_fpr, query_time, "0");
}

fn report_fpr_3<T: MQ>(n: usize, fpr: f32, amq: &mut T) {
    let min_not_inserted = n;

    for i in 0..(n) {
        amq.insert(&i);
    }
    let (e_fpr, query_time) = fpr_and_query_time(n, min_not_inserted, amq);
    println!("{}\t{}\t{}\t{}\t{}",n, fpr, e_fpr, query_time, "100");
}
// fn report_fpr_2(N: usize, fpr: f32) {
//     let mut bf = BloomFilter::with_fpr(fpr, N);
//     for i in 0..(N/3 * 2) {
//         bf.insert(&i);
//     }

//     let N_queries = N - (N/3);
//     let mut fps = 0;
//     for i in ((2*N/3)..N) {
//         let isin = bf.query(&i);
//         if isin {
//             fps += 1;
//         }
//     }
//     let e_fpr = fps as f32 / N_queries as f32;
//     println!("{}\t{}\t{}\t{}",N, fpr, e_fpr, "0.5");
// }

// fn report_fpr_3(N: usize, fpr: f32) {
//     let mut bf = BloomFilter::with_fpr(fpr, N);
//     for i in 0..N {
//         bf.insert(&i);
//     }

//     let N_queries = N - (N/3);
//     let mut fps = 0;
//     for i in (0..N) {
//         let isin = bf.query(&i);
//         if isin {
//             fps += 1;
//         }
//     }
//     let e_fpr = fps as f32 / N_queries as f32;
//     println!("{}\t{}\t{}\t{}", N, fpr, e_fpr, "100%");
// }