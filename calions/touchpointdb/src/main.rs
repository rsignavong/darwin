use memmap::Mmap;
use rayon::iter::split as par_split;
use rayon::prelude::*;
// use std::fs::File;
use std::fs::OpenOptions;
use std::path::PathBuf;
// use std::sync::mpsc::channel;
use std::time::Instant;
use std::{thread, time};
use twoway::find_bytes;

fn main() {
    let bench = Instant::now();

    let path: PathBuf = PathBuf::from("touchpoints.db");
    let file = OpenOptions::new().read(true).open(path).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };

    let _: Vec<Option<String>> = par_split(&mmap[..], |bytes| match find_bytes(bytes, b"\n") {
        Some(position) => (&bytes[0..position], Some(&bytes[position + 1..bytes.len()])),
        None => (&bytes[0..bytes.len()], None),
    })
    .map(|bytes| match find_bytes(bytes, b":") {
        Some(offset) => {
            let (key, value) = bytes.split_at(offset);
            if key == b"0000000000000000" {
                let string_value = String::from_utf8(value.to_vec()).unwrap();
                println!("found: {}", string_value.to_owned());
                Some(string_value)
            } else {
                None
            }
        }
        None => None,
    })
    .collect();
    println!("{} us", bench.elapsed().as_micros());
    thread::sleep(time::Duration::from_millis(5000));
}
