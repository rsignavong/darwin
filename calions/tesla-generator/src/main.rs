mod generator;

fn main() -> std::io::Result<()> {
    let (output, max) = generator::cli::build();
    generator::data::gen(&output, max)
}
