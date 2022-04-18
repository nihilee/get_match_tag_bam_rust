use rust_htslib::{bam, bam::Read};
use std::env;
use std::str;
use std::fs;
use std::io::{BufRead, BufReader};

extern crate rustc_hash;
use rustc_hash::{FxHashMap, FxHashSet};

// 原来从根上就不行，就当我周末做了个梦吧哈哈

fn encode(seq: String) -> Vec<u8> {
    let m:FxHashMap<&str, u8> = vec![
        ("AA", 17),
        ("AC", 18),
        ("AG", 20),
        ("AT", 24),
        ("AN", 31),
        ("CA", 33),
        ("CC", 34),
        ("CG", 36),
        ("CT", 40),
        ("CN", 47),
        ("GA", 65),
        ("GC", 66),
        ("GG", 68),
        ("GT", 72),
        ("GN", 79),
        ("TA", 129),
        ("TC", 130),
        ("TG", 132),
        ("TT", 136),
        ("TN", 143),
        ("NA", 241),
        ("NC", 242),
        ("NG", 244),
        ("NT", 248),
        ("NN", 255),
        ("A", 16),
        ("C", 32),
        ("G", 64),
        ("T", 128),
        ("N", 240),
    ].into_iter().collect();

    let mut o = vec![];
    for i in seq.as_bytes().chunks(2).map(str::from_utf8) {
        o.push(m.get(i.unwrap()).unwrap().clone())
    }
    println!("{:?}", o);
    o
}


fn main() {
    // println!("Hello, world!");
    let args:Vec<String> = env::args().collect();

    let f = fs::File::open(&args[1]).unwrap();
    let reader = BufReader::new(f);

    // read bam instead of text
    // let mut tag = bam::Reader::from_path(&args[1]).unwrap();
    let mut s = FxHashSet::default();
    // for r in tag.records() {
        // let record = r.unwrap();
        // println!("record: {:?}", record.seq().encoded);
        // s.insert(record.seq().encoded.to_vec());
    // }

    // i know how to convert string to 4 bits
    for line in reader.lines() {
        // let b: Vec<u8> = line.unwrap().as_bytes().to_vec();
        if let Ok(li) = line {
            // for base in "ACGT".chars() {
            //     // println!("lb: {}{}", base, li);
            //     // let lb: Vec<u8> = encode(format!("{}{}", base, li));
            //     s.insert(encode(format!("{}{}", base, li)));
            // }
            // println!("b: {}", li);
            s.insert(li.as_bytes().to_owned());
            // for base in [
            //     "AA","AC","AG","AT",
            //     "CA","CC","CG","CT",
            //     "GA","GC","GG","GT",
            //     "TA","TC","TG","TT",
            // ] {
            //     println!("sb: {}{}", base, li);
            //     println!("sb: {}{}", li, base);
            //     // let sb: Vec<u8> = encode(format!("{}{}", base, li));
            //     s.insert(encode(format!("{}{}", base, li)));
            //     s.insert(encode(format!("{}{}", li, base)));
            // }
        }
    }


    let mut bam = bam::Reader::from_path(&args[2]).unwrap();
    let header = bam::Header::from_template(bam.header());
    let mut out = bam::Writer::from_path(&args[3], &header, bam::Format::Bam).unwrap();
    bam.set_threads(20);

    for r in bam.records() {
        let record = r.unwrap();
        // println!("c: {:?}", record.seq().encoded);

        // let slice = record.seq().encoded;
        // if s.contains(&slice[slice.len()-20..]) {
            // out.write(&record).unwrap();
        // }

        for c in record.seq().as_bytes().windows(38){
            // println!("c: {:?}", c);
            if s.contains(c){
                out.write(&record).unwrap();
                break
            }
        }
    }
}