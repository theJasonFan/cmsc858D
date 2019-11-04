
use succinct::wt::WT;
use std::env;
use serde::{Serialize, Deserialize};

use std::fs::{self, File};
use std::io::{self, prelude::*, BufReader};
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    let subparser = &args[1];
    assert_eq!(args.len(), 4);
    if subparser == "build" {
        build(&args[2], &args[3])
    } else if subparser == "access" {
        access(&args[2], &args[3])
    } else if subparser == "rank" {
        rank(&args[2], &args[3])
    } else if subparser == "select" {
        select(&args[2], &args[3])
    } else {
        println!("{} - not implemented", subparser);
    }
}

fn build(s: &String, fp: &String) {
    let wt = WT::new(s);
    let encoded = bincode::serialize(&wt).unwrap();

    println!("encoded {}", encoded.len());
    println!("size {}", wt.size_of());

    fs::write(fp, &encoded).expect("Error");

}

fn access(wt_path: &String, fp: &String) {
    let wt = load_wt(wt_path);

    let file = File::open(fp).expect("Error");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let s = line.expect("Error");
        let i = s.parse::<usize>().expect("line not an int");
        println!("{}", wt.access(i));
    }
}

fn rank(wt_path: &String, fp: &String)  {
    let wt = load_wt(wt_path);

    let file = File::open(fp).expect("Error");
    let reader = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(reader);
    for record in rdr.records() {
        let r = record.expect("Error");
        // let i = s.parse::<usize>().expect("line not an int");
        let c = r[0].chars().next().expect("Cannot parse character");
        let i = r[1].parse::<usize>().expect("Cannot parse index");
        println!("{}", wt.rank(c, i));
    }
}

fn select(wt_path: &String, fp: &String)  {
    let wt = load_wt(wt_path);

    let file = File::open(fp).expect("Error");
    let reader = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(reader);
    for record in rdr.records() {
        let r = record.expect("Error");
        // let i = s.parse::<usize>().expect("line not an int");
        let c = r[0].chars().next().expect("Cannot parse character");
        let i = r[1].parse::<usize>().expect("Cannot parse index");
        println!("{}", wt.select(c, i).expect("Not found"));
    }
}

fn load_wt(fp: &String) -> WT {
    let data = fs::read(&fp).expect("Error");
    let wt: WT = bincode::deserialize(&data[..]).unwrap();
    wt
}