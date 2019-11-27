use succinct::bloom_filter::{BlockedBloomFilter, BloomFilter, MQ};
use succinct::math::{clog, exp2};
use std::hash::Hash;

use std::env;
use std::time::Instant;
use std::cmp::min;



fn main() {
    /*
    USAGE:
        ./bf_bench
    */
    
    // Sample Ns from log space
    let n_ns = 20;
    // let max_n = 5000000;
    let max_n = 10000000;
    let min_n = 1000;

    let log_max_n = (max_n as f32).ln();
    let log_min_n = (min_n as f32).ln();

    let log_incr_n = (log_max_n - log_min_n) / (n_ns - 1) as f32;
    let mut ns: Vec<usize> = (0..n_ns).map(|x| (x as f32 * log_incr_n + log_min_n).exp() as usize).collect();
    ns[n_ns - 1] = max_n;

    let n_fprs = 10;
    let min_fpr = 0.01;
    let max_fpr = 0.25;

    let incr_fpr = (max_fpr - min_fpr) / (n_fprs - 1) as f32;
    let fprs: Vec<f32> = (0..n_fprs).map(|x| x as f32 * incr_fpr + min_fpr).collect();

    println!( "{}\t{}\t{}\t{}\t{}\t{}\t{}", "N", "FPR", "empirical_FPR",  "query_time", "frac_query_inserted", "amq", "amq_size");

    for n in &ns {
        for fpr in &fprs {
            let insert_up_to = *n;
            let mut bf = BloomFilter::with_fpr(*fpr, *n);
            let mut bbf = BlockedBloomFilter::with_fpr(*fpr, *n, 64_usize);
            insert_amq_up_to(insert_up_to, &mut bf);
            insert_amq_up_to(insert_up_to, &mut bbf);

            let window_size = min(min_n, 5000);
            let query_from_0 = insert_up_to;
            let query_to_0 = insert_up_to + window_size;
            let query_from_50 = insert_up_to - (window_size / 2);
            let query_to_50 = insert_up_to + (window_size / 2);
            let query_from_100 = 0;
            let query_to_100 = window_size;

            let bf_n = bf.len();
            let bbf_n = bbf.len();

            let (e_fpr, ns_per_query) = benchmark_amq(insert_up_to, query_from_0, query_to_0, &bf);
            println!( "{}\t{}\t{}\t{}\t{}\t{}\t{}", n, fpr, e_fpr, ns_per_query, "0.", "bf", bf_n);

            let (e_fpr, ns_per_query) = benchmark_amq(insert_up_to, query_from_0, query_to_0, &bbf);
            println!( "{}\t{}\t{}\t{}\t{}\t{}\t{}", n, fpr, e_fpr, ns_per_query, "0.", "bbf", bbf_n);

            let (e_fpr, ns_per_query) = benchmark_amq(insert_up_to, query_from_50, query_to_50, &bf);
            println!( "{}\t{}\t{}\t{}\t{}\t{}\t{}", n, fpr, e_fpr, ns_per_query, "0.5", "bf", bf_n);

            let (e_fpr, ns_per_query) = benchmark_amq(insert_up_to, query_from_50, query_to_50, &bbf);
            println!( "{}\t{}\t{}\t{}\t{}\t{}\t{}", n, fpr, e_fpr, ns_per_query, "0.5", "bbf", bbf_n);

            let (e_fpr, ns_per_query) = benchmark_amq(insert_up_to, query_from_100, query_to_100, &bf);
            println!( "{}\t{}\t{}\t{}\t{}\t{}\t{}", n, fpr, e_fpr, ns_per_query, "1.", "bf", bf_n);

            let (e_fpr, ns_per_query) = benchmark_amq(insert_up_to, query_from_100, query_to_100, &bbf);
            println!( "{}\t{}\t{}\t{}\t{}\t{}\t{}", n, fpr, e_fpr, ns_per_query, "1.", "bbf", bbf_n);
        }
    }
}

fn timed_query<H: Hash, T: MQ>(x: &H, amq: &T) -> (bool, usize) {
    let t = Instant::now();
    let isin = amq.query(x);
    let elapsed = t.elapsed().as_nanos() as usize;
    (isin, elapsed)
}

fn insert_amq_up_to<T: MQ>(insert_up_to: usize, amq: &mut T) {
    for i in 0..(insert_up_to) {
        amq.insert(&i);
    }
}
fn benchmark_amq<T: MQ>(insert_up_to: usize, query_from: usize, query_to: usize, amq: &T) -> (f32, f32) {
    let mut n_true_neg = 0;
    let mut n_false_pos = 0;
    let mut total_elapsed = 0;
    let mut n_queries = 0;

    for i in query_from..query_to {
        n_queries += 1;
        let (isin, elapsed) = timed_query(&i, amq);

        if i >= insert_up_to {
            if isin {
                n_false_pos += 1;
            }
            n_true_neg += 1;
        }
        total_elapsed += elapsed;
    }

    let fpr = n_false_pos as f32 / n_true_neg as f32;
    let ns_per_query = total_elapsed as f32 / n_queries as f32;
    
    (fpr, ns_per_query)
}