use rust_htslib::{bam, bam::Read};
use std::env;
// use std::str;
use std::fs;
use std::io::{self, BufRead};
// use std::error;

extern crate fxhash;
use fxhash::FxHashSet;


fn main() {
    // println!("Hello, world!");

    let args:Vec<String> = env::args().collect();
    // let a = tag_set(&args[1]).unwrap();
    // println!("{:?}", a);

    let f = fs::File::open(&args[1]).unwrap();
    let reader = io::BufReader::new(f);
    let mut s = FxHashSet::default();

    for line in reader.lines() {
        // println!("{:?}", line.unwrap());
        let b: Vec<u8> = line.unwrap().as_bytes().to_vec();
        // println!("from abc {:?}", b);
        // print_type_of(&b);
        s.insert(b);
        // break
    }

    let mut bam = bam::Reader::from_path(&args[2]).unwrap();
    let header = bam::Header::from_template(bam.header());
    let mut out = bam::Writer::from_path(&args[3], &header, bam::Format::Bam).unwrap();

    for r in bam.records() {
        let record = r.unwrap();
        // println!("{:#?}", record.seq().as_bytes().chunks(38));
        for c in record.seq().as_bytes().windows(38){
            // println!("{:?}, {:?}, {:?}", c, s.contains(c), s);
            // print_type_of(&c);
            if s.contains(c){
                // println!("{:?}", str::from_utf8(c));
                // println!("{:?}", str::from_utf8(&record.seq().as_bytes()).unwrap());
                out.write(&record).unwrap();
                break
            }
        }
    }
}

// fn tag_set(t: &str) -> Result<FxHashSet<String>, Box<dyn error::Error>> {
//     let a = fs::File::open(t)?;
//     let reader = io::BufReader::new(a);
//     // let mut b = HashSet::new();
//     let mut b = FxHashSet::default();

//     for line in reader.lines() {
//         b.insert(line?);
//     }

//     Ok(b)
// }

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
