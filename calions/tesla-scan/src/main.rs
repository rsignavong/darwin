mod scan;

fn main() -> std::io::Result<()> {
    let (db, poc) = scan::cli::build();
    scan::find::find(&db, &poc)
}
