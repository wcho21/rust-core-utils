use clap::{Arg, ArgAction, Command};

fn main() {
    let (text, omit_newline) = get_args();

    echo_r::echo(&text, omit_newline);
}

fn get_args() -> (Vec<String>, bool) {
    let text = Arg::new("text")
        .value_name("TEXT")
        .help("Input text")
        .num_args(1..)
        .required(true);

    let omit_newline = Arg::new("omit_newline")
        .short('n')
        .help("Do not print the trailing newline character.")
        .num_args(0)
        .action(ArgAction::SetTrue);

    let matches = Command::new("echo_r")
        .version("0.1.0")
        .arg(text)
        .arg(omit_newline)
        .get_matches();

    let text: Vec<String> = matches
        .get_many::<String>("text")
        .unwrap()
        .cloned()
        .collect();
    let omit_newline = matches.get_flag("omit_newline");

    (text, omit_newline)
}