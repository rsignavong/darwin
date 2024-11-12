use memmap::Mmap;
use bstr::ByteSlice;
use std::{fs::OpenOptions, path::PathBuf, time::Instant};

pub fn find(db: &str, poc: &str) -> std::io::Result<()> {
    let now = Instant::now();
    let path: PathBuf = PathBuf::from(db);
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Cannot open file database");
    let mmap = unsafe { Mmap::map(&file).expect("Cannot create Memory mapping to file") };

    if let Some(offset) = &mmap[..].find(poc.as_bytes()) {
        let ulid_offset = offset + poc.len() + 1;
        let string_value = String::from_utf8(mmap[ulid_offset..ulid_offset + 26].to_vec())
            .expect("Unable to cast value to string");
        println!("found: {}", string_value.to_owned());
    } else {
        println!("Not found");
    }

    println!("File scanned in {} µs", now.elapsed().as_micros());
    Ok(())
}
