use clap::{App, Arg};

pub fn build() -> (String, usize) {
    let args = App::new("Tesla Generator")
        .version("1.0.0")
        .author("Rocky S. <rsignavong@gmail.com>")
        .about("Tesla DB data generator")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("File name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .value_name("NUMBER")
                .help("Number of lines to generate")
                .takes_value(true),
        )
        .get_matches();

    let output = args.value_of("output").unwrap_or("tesla");
    let number = args.value_of("number").unwrap_or("100");

    (
        output.to_owned(),
        number
            .to_owned()
            .parse::<usize>()
            .expect("Expected a number"),
    )
}
