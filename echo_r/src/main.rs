use clap::{App, Arg};

const SEPARATOR: &str = " ";

fn main() {
    // define args
    let text_arg = Arg::with_name("text")
        .value_name("TEXT")
        .help("Input text")
        .required(true)
        .min_values(1);

    let omit_newline_arg = Arg::with_name("omit_newline")
        .short("n")
        .help("Do not print the trailing newline character.")
        .takes_value(false);

    // parse args
    let matches = App::new("echo_r")
        .version("0.1.0")
        .arg(text_arg)
        .arg(omit_newline_arg)
        .get_matches();

    let text_matched = matches.values_of_lossy("text").unwrap();
    let omit_newline_matched = matches.is_present("omit_newline");

    // apply args
    let ending = if omit_newline_matched { "" } else { "\n"};

    // print
    print!("{}{}", text_matched.join(SEPARATOR), ending);
}
