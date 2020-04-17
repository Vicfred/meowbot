#![forbid(unsafe_code)]

use clap;
use env_logger;
use irc_bot;
use log;

use ansi_term::Colour::{Blue, Red, Yellow};
use clap::{arg_enum, value_t};
use irc_bot::modules;

fn main() {
    println!("This is in red: {}", Red.paint("a red string"));
    println!("Demonstrating {} and {}!",
         Blue.bold().paint("blue bold"),
         Yellow.underline().paint("yellow underline"));

    println!("Yellow on blue: {}", Yellow.on(Blue).paint("wow!"));
    let args = clap::App::new("egbot")
        .arg(
            clap::Arg::with_name("config-file")
                .long("config-file")
                .short("c")
                .default_value("config.yaml"),
        )
        .arg(
            clap::Arg::with_name("data-dir")
                .long("data-dir")
                .short("d")
                .default_value("data"),
        )
        .arg(
            clap::Arg::with_name("error-verbosity")
                .long("error-verbosity")
                .possible_values(&ErrorVerbosity::variants())
                .case_insensitive(true)
                .default_value("Display"),
        )
        .get_matches();

    env_logger::init();

    let error_verbosity =
        value_t!(args, "error-verbosity", ErrorVerbosity).unwrap_or_else(|err| err.exit());
}

arg_enum! {
    #[derive(Debug)]
    enum ErrorVerbosity {
        Display,
        Debug
    }
}

