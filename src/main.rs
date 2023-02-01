use std::env;
use std::fs;
use std::io::Read;

use crate::bytes::Bytes;
use crate::roland::RD300NX;

mod roland;
mod bits;
mod bytes;

fn main() {
    let mut args = env::args().skip(1);
    let filename = args.next().expect("The first argument should be the RDS file");
    let mut bytes = Vec::new();
    let mut f = fs::File::options().read(true).open(&filename).expect(&format!("File could not be read: {}", filename));
    let size = f.read_to_end(&mut bytes).unwrap();
    if size != RD300NX::BYTE_SIZE {
        println!("File should be {} bytes", RD300NX::BYTE_SIZE);
    } else {
        let rds = RD300NX::parse(bytes.try_into().unwrap()).unwrap();
        for (i, ls) in rds.user_sets.iter().enumerate() {
            println!("#{} {}", i + 1, ls.name_string());
        }
    }
}
