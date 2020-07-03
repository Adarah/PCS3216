use clap::{App, Arg, SubCommand};
use sisprog::{Assembler, Config, CPU};
use std::env;

fn main() {
    let matches = App::new("PCS3216")
        .version("1.0")
        .author("Lucas Harada <lucasyharada@gmail.com>")
        .about("Simple VM that assembles and runs arbitrary code")
        .subcommand(
            SubCommand::with_name("cpu")
                .arg(
                    Arg::with_name("OUTPUT")
                        .value_name("OUTPUT FILE")
                        .required(true)
                        .help("Location to save output of program")
                        .index(1),
                )
                .arg(
                    Arg::with_name("INPUT")
                        .help("Absolute object code to be run")
                        .value_name("INPUT FILE")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name("LOADER")
                        .value_name("LOADER FILE")
                        .short("L")
                        .required(true)
                        .help("Loader to be used. Must be in a binary format"),
                )
                .arg(
                    Arg::with_name("v")
                        .short("v")
                        .multiple(true)
                        .help("Sets the level of verbosity"),
                ),
        )
        .subcommand(
            SubCommand::with_name("assembler")
                .arg(
                    Arg::with_name("OUTPUT")
                        .value_name("OUTPUT FILE")
                        .required(true)
                        .help("Location to save object code")
                        .index(1),
                )
                .arg(
                    Arg::with_name("INPUT")
                        .help("ASM file to be assembled")
                        .value_name("INPUT FILE")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches();

    let key = "RUST_LOG";

    if let Some(matches) = matches.subcommand_matches("cpu") {
        match matches.occurrences_of("v") {
            0 => (),
            1 => env::set_var(key, "info".to_string()),
            2 => env::set_var(key, "debug".to_string()),
            _ => env::set_var(key, "trace".to_string()),
        }
        pretty_env_logger::init();
        let inp = matches.value_of("INPUT").unwrap().to_string();
        let out = matches.value_of("OUTPUT").unwrap().to_string();
        let loader = matches.value_of("LOADER").unwrap().to_string();
        let conf = Config::new(inp, out, loader, false);
        CPU::run(conf);
    } else if let Some(matches) = matches.subcommand_matches("assembler") {
        let inp = matches.value_of("INPUT").unwrap().to_string();
        let out = matches.value_of("OUTPUT").unwrap().to_string();
        Assembler::run(inp, out);
    }
}
