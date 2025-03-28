use clap::{Arg, ArgAction, Command};

fn main() {
    // define args
    let text_arg = Arg::new("text")
        .value_name("TEXT")
        .help("Input text")
        .num_args(1..)
        .required(true);

    let omit_newline_arg = Arg::new("omit_newline")
        .short('n')
        .help("Do not print the trailing newline character.")
        .num_args(0)
        .action(ArgAction::SetTrue);

    // parse args
    let matches = Command::new("echo_r")
        .version("0.1.0")
        .arg(text_arg)
        .arg(omit_newline_arg)
        .get_matches();

    let text_matched: Vec<&str> = matches
        .get_many::<String>("text")
        .unwrap()
        .map(String::as_str)
        .collect();
    let omit_newline_matched = matches.get_flag("omit_newline");

    echo_r::echo(text_matched, omit_newline_matched);
}
