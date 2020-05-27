use clap::{App, Arg};
use sisprog::{Assembler, Config, CPU};
use std::env;


fn main() {

    let matches = App::new("PCS3216")
        .version("1.0")
        .author("Lucas Harada <lucasyharada@gmail.com>")
        .about("Simple VM that assembles and runs arbitrary code")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input asm file to use")
                .value_name("INPUT FILE")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let key = "RUST_LOG";
    match matches.occurrences_of("v") {
        1 => env::set_var(key, "info".to_string()),
        2 => env::set_var(key, "debug".to_string()),
        3 => env::set_var(key, "trace".to_string()),
        _ => (),
    }
    pretty_env_logger::init();

    Assembler::run(matches.value_of("INPUT").unwrap().to_string());
    let conf = Config::new(false);
    CPU::run(conf);
}
