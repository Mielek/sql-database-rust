use std::io::{stdout, stdin};
use std::process::exit;
use std::io::Write;

const EXIT_SUCCESS: i32 = 0;

fn print_prompt() {
    print!("db > ");
    stdout().flush().unwrap();
}

fn main() {
    loop {
        print_prompt();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Expected line of input.");
        let command = input.trim();

        if ".exit" == command.trim() {
            exit(EXIT_SUCCESS);
        } else {
            println!("Unrecognized command '{0}'.", command);
        }
    }
}
