use clap::{App, Arg};

pub fn build() -> (String, String) {
    let args = App::new("Tesla Scanner")
        .version("1.0.0")
        .author("Rocky S. <rsignavong@gmail.com>")
        .about("Tesla DB data scanner")
        .arg(
            Arg::with_name("database")
                .short("d")
                .long("database")
                .value_name("FILE")
                .help("Tesla database")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("poc")
                .short("p")
                .long("poc")
                .value_name("EMAIL")
                .help("Point of Contact")
                .takes_value(true),
        )
        .get_matches();

    let db = args.value_of("database").unwrap_or("tesla.db");
    let poc = args.value_of("poc").unwrap_or("user0@mail.com");

    (db.to_owned(), poc.to_owned())
}
