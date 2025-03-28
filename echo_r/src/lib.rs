const SEPARATOR: &str = " ";

pub fn echo(text: Vec<&str>, omit_newline: bool) {
    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(SEPARATOR), ending);
}