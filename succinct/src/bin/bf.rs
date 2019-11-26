
use succinct::bloom_filter::{BloomFilter, MQ};
use std::env;

use std::fs::{self, File};
use std::io::{prelude::*, BufReader};


fn main() {
    /* Bloom filter app */
    let args: Vec<String> = env::args().collect();
    let subparser = &args[1];
    if subparser == "build" {
        // Usage 
        //   bf build <key_file> <fpr> <n distinct keys> <output>
        assert_eq!(args.len(), 6);
        let key_file = &args[2];
        let fpr: f32 = args[3].parse().unwrap();
        let n_keys: usize = args[4].parse().unwrap();
        let output = &args[5];

        build(key_file, fpr, n_keys, output);

    } else if subparser == "query" {
        // Usage 
        //   bf build <bloom_filter> <queries>
        assert_eq!(args.len(), 4);
        let bf = &args[2];
        let queries = &args[3];
        query(bf, queries);
    } else {
        println!("{} - not implemented", subparser);
    }
}

fn build(key_file: &str, fpr: f32, n_keys: usize, out_file: &str) {
    let file = File::open(key_file).unwrap();
    let reader = BufReader::new(file);
    
    let mut bf = BloomFilter::with_fpr(fpr, n_keys);
    for line in reader.lines() {
        let line = line.unwrap();
        bf.insert(&line);
    }

    let encoded = bincode::serialize(&bf).unwrap();
    fs::write(out_file, &encoded).expect("Failed to write output");
}

fn query(bf_fp: &str, query_file: &str) {
    let data = fs::read(&bf_fp).expect("Error");
    let bf: BloomFilter = bincode::deserialize(&data[..]).unwrap();

    let file = File::open(query_file).unwrap();
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line.unwrap();
        let isin = bf.query(&line);
        let ans;
        if isin {
            ans = 'Y'
        } else {
            ans = 'N'
        }
        println!("{}:{}", line, ans);
    }
}