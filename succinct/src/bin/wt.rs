
use succinct::wt::WT;
use std::env;

use std::fs::{self, File};
use std::io::{prelude::*, BufReader};


fn main() {
    /* Wavelet Tree APP
   
    */
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

fn build(in_file: &String, out_file: &String) {
    /* Build a wavelet tree from a string containing an input text.

    USAGE:
        $wt build <input string> <output file>
    
    This command reads the string in <input file>, constructs the wavelet tree, 
    and saves the resulting structure to the file <output file>.
    
    The program should also write two lines to standard out; 
        - the first line should contain the number of distinct input characters
          in the <input string> file 
        - the second line should contain the number of characters in the input 
          string. The command should be executed as follows:
    */
    let s = fs::read_to_string(in_file).expect("Failed to read input");
    let wt = WT::new(&s);
    let encoded = bincode::serialize(&wt).unwrap();

    println!("{}", wt.n_chars());
    println!("{}", wt.len());

    fs::write(out_file, &encoded).expect("Failed to write output");
}

fn access(wt_path: &String, fp: &String) {
    /* Load a wavelet tree from file, and issue a series of access queries on 
       the supplied indices
    
    USAGE:
        $wt access <saved wt> <access indices>
    
    ARGUMENTS:
        <saved wt>: the serialized wavelet tree from `build`
        <access indices>: newline-separated list of indices (0-based) to access
    
    OUTPUT:
        Characters (one per-line) corresponding to each index in the file 
        <access indices> to standard out.
    */
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
    /* Load a wavelet tree from file, and issue a series of rank queries on 
       the supplied indices
    
    USAGE:
        $wt access <saved wt> <rank queries>
    
    ARGUMENTS:
        <saved wt>: the serialized wavelet tree from `build`
        <rank queries>: newline-separated, tab seperated tuples of <c>\t<i> 
            for char c and index i.
    
    OUTPUT:
        Characters (one per-line) corresponding to each rank query in the file 
        <rank query> to standard out.
    */
    let wt = load_wt(wt_path);

    let file = File::open(fp).expect("Error");
    let reader = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(reader);
    for record in rdr.records() {
        let r = record.expect("Error");
        let c = r[0].chars().next().expect("Cannot parse character");
        let i = r[1].parse::<usize>().expect("Cannot parse index");
        println!("{}", wt.rank(c, i));
    }
}

fn select(wt_path: &String, fp: &String)  {
    /* Load a wavelet tree from file, and issue a series of select queries on 
       the supplied indices
    
    USAGE:
        $wt access <saved wt> <select queries>
    
    ARGUMENTS:
        <saved wt>: the serialized wavelet tree from `build`
        <select queries>: newline-separated, tab seperated tuples of <c>\t<i> 
            for char c and index i.
    
    OUTPUT:
        Characters (one per-line) corresponding to each select query in the file 
        <select query> to standard out.
    */
    let wt = load_wt(wt_path);

    let file = File::open(fp).expect("Error");
    let reader = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_reader(reader);
    for record in rdr.records() {
        let r = record.expect("Error");
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