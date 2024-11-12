use std::{
    fs::OpenOptions,
    io::{LineWriter, Write},
    time::Instant,
};
use ulid::Ulid;

pub fn gen(output: &str, max: usize) -> std::io::Result<()> {
    let now = Instant::now();
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}.db", output))?;
    let mut file = LineWriter::new(file);
    for n in 0..max {
        let _ = file.write_all(format!("user{}@mail.com:{}\n", n, Ulid::new()).as_bytes());
    }

    println!("File created in {} µs", now.elapsed().as_micros());
    Ok(())
}
