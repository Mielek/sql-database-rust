use std::io::{stdin, stdout};
use std::io::Write;
use std::process::exit;

const EXIT_SUCCESS: i32 = 0;

fn print_prompt() {
    print!("db > ");
    stdout().flush().unwrap();
}

fn read_line() -> String {
    print_prompt();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Expected line of input.");
    return input.trim().to_string();
}

fn is_meta_command(input_line: &str) -> bool {
    return input_line.starts_with('.');
}

fn do_meta_command(input_line: &str) -> Result<(), String> {
    match input_line {
        ".exit" => exit(EXIT_SUCCESS),
        _ => Err(format!("Unrecognized command '{}'.", input_line)),
    }
}

fn prepare_statement(input_line: &str) -> Result<(), String> {
    if input_line.starts_with("insert") {
        return Ok(());
    }
    if input_line.starts_with("select") {
        return Ok(());
    }
    return Err(format!("Unrecognized keyword at start of '{}'.", input_line));
}

fn execute_statement(_statement: &()) {
    println!("Executed.");
}

fn main() {
    loop {
        let input_line = read_line();

        if is_meta_command(&input_line) {
            do_meta_command(&input_line).err().map(|e: String| println!("{}", e));
            continue;
        }

        match prepare_statement(&input_line) {
            Ok(statement) => execute_statement(&statement),
            Err(e) => println!("{}", e),
        }
    }
}
