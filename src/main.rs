use rust_htslib::{bam, bam::Read};
use std::env;
use std::str;
use std::fs;
use std::io::{BufRead, BufReader};

extern crate rustc_hash;
use rustc_hash::{FxHashMap, FxHashSet};


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
    ].into_iter().collect();

    let mut o = vec![];
    for i in seq.as_bytes().chunks(2).map(str::from_utf8) {
        o.push(m.get(i.unwrap()).unwrap().clone())
    }

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
        //原来如此，unwrap不能乱用，因为用完unwrap就失去了处理里面的Ok或者Some的机会
        //之前这里就是不能clone了，因为不能直接克隆Result,只能克隆里面的String,
        // 但是和unwrap写在一行的时候就已经移动了
        if let Ok(li) = line {
            for base in "ACGTN".chars() {
                let lb: Vec<u8> = encode(format!("{}{}", base, li));
                s.insert(lb);
            }
            for base in [
                "AA","AC","AG","AT","AN",
                "CA","CC","CG","CT","CN",
                "GA","GC","GG","GT","GN",
                "TA","TC","TG","TT","TN",
                "NA","NC","NG","NT","NN",
            ] {
                let sb: Vec<u8> = encode(format!("{}{}", base, li));
                s.insert(sb);
            }
        }
    }

    // println!("s: {:?}", s);

    let mut bam = bam::Reader::from_path(&args[2]).unwrap();
    let header = bam::Header::from_template(bam.header());
    let mut out = bam::Writer::from_path(&args[3], &header, bam::Format::Bam).unwrap();
    bam.set_threads(20);

    for r in bam.records() {
        let record = r.unwrap();
        for c in record.seq().encoded.windows(20){
            // println!("c: {:?}", c);
            if s.contains(c){
                out.write(&record).unwrap();
                break
            }
        }
    }
}