#![deny(warnings)]
extern crate clap;

use clap::{App, Arg, ArgMatches};

fn argparse<'a>() -> ArgMatches<'a> {
    App::new("echo")
        .about("echo executable for testing")
        .arg(Arg::with_name("n")
                .short("n")
                .help("do not append newline")
                .takes_value(false)
                .required(false),
        )
        .arg(Arg::with_name("arg")
            .help("arg to echo"))
        .get_matches()
}

fn main() {
    let matches = argparse();
    let echo_msg = match matches.values_of_lossy("arg") {
        None => String::from(""),
        Some(x) => {
            let v = x.into_iter().collect::<Vec<_>>();
            v.join(" ")
        }
    };
    if matches.is_present("n") {
        print!("{}", echo_msg);
    } else {
        println!("{}", echo_msg);
    }
}
