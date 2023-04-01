use std::io::{Result, BufRead, Write};

use scanner::Scanner;

mod error_log;
mod scanner;
mod tokens;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    if args.len() > 1 {
        eprintln!("Usage: rlox [script]");
    } else if args.len() == 1 {
        run_script(args.next().expect("a script to run"))?;
    } else {
        start_repl()?;
    }

    Ok(())
}

//Read a script to string, and then run it
fn run_script(path: String) -> Result<()> {
    let script = std::fs::read_to_string(path)?;
    run(script)
}

//Accept a single line of code at a time, executing as it's read
fn start_repl() -> Result<()> {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let mut buffer = String::new();
        handle.read_line(&mut buffer)?;
        let buffer = buffer.trim().to_owned();
        if &buffer == "/quit" {
            break;
        }

        run(buffer)?;
    }

    Ok(())
}

//Run the script in string form.
fn run(script: String) -> Result<()> {
    let scanner = Scanner::new(script);
    let tokens = scanner.scan_tokens();

    tokens.into_iter()
        .for_each(|token| println!("{token:?}"));
    Ok(())
}
