use std::env;
use std::fs;
use std::io::Read;

use crate::bits::BitStream;
use crate::roland::RD300NX;

mod roland;
mod bits;

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().expect("The first argument should be the RDS file");
    let mut bytes = Vec::new();
    let mut f = fs::File::options().read(true).open(&filename).expect(&format!("File could not be read: {}", filename));
    let size = f.read_to_end(&mut bytes).unwrap();
    println!("Read {} bytes", size);
    let stream: BitStream = bytes.into_iter().into();
    let rds = RD300NX::parse(stream).unwrap();
    for (i, ls) in rds.live_sets.iter().enumerate() {
        println!("#{} {}", i, ls.name_string());
    }
}
