use std::{
    env::args, fs, io::{stdin, stdout, Write}, process::exit
};
use error::Error;

mod error;
mod scanner;
mod token;

fn main() {
    let mut args = args();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    }
    _ = args.next();
    if let Some(file) = args.next() {
        run_file(&file);
    } else {
        run_prompt();
    }
}

fn run_prompt() {
    println!("Lox REPL");

    print!("> ");
    _ = stdout().flush();
    stdin().lines().flatten().for_each(|line| {
	if line.starts_with('!') {
	    handle_command(&line[1..]);
	} else {
            if let Err(err) = run(line) {
		eprintln!("{err}");
            }
	}
	print!("> ");
        _ = stdout().flush();
    });
}

fn run_file(path: &String) {
    if let
	Err(_) = run(fs::read_to_string(path)
		     .expect(&format!("Failed to read  from {}", path))) {
        exit(65)
    }
}

fn run(source: String) -> Result<(), Error> {
    match scanner::scan_tokens(source) {
	Ok(tokens) => {
	    tokens.iter().for_each(|token| println!("{token:?}"));
	    Ok(())
	},
	Err(errors) => Err(Error::SyntaxErrors(errors))
    }

}

fn handle_command(command: &str) {
    match command {
	"clear" => print!("\x1B[2J\x1B[1;1H"),
	_ => println!("Unknwon command {}", command),
    }
}
