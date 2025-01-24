mod interpreter;
mod lexer;
mod parser;
mod utils;

use lexer::{Scanner, Token};
use parser::{AstPrinter, Expr, Parser};
use std::{env, fs, io, io::Write, process};

fn main() {
    let mut args: env::Args = env::args();
    args.next();

    // Global Error state
    let mut had_err: bool = false;

    match args.len() {
        0 => run_prompt(&mut had_err),
        1 => run_file(&args.next().unwrap(), &mut had_err),
        _ => {
            eprintln!("Usage: rlox [script]");
            process::exit(64);
        }
    }
    .expect("");
}

fn run_file(path: &str, had_err: &mut bool) -> Result<(), io::Error> {
    let source: String = fs::read_to_string(path)?;

    run(source, had_err);
    if *had_err {
        process::exit(65);
    }
    Ok(())
}

fn run_prompt(had_err: &mut bool) -> Result<(), io::Error> {
    let mut prompt = String::new();

    loop {
        print!(">");

        match io::stdout().flush() {
            Ok(_) => (),
            Err(err) => eprintln!("Error while flushing stdout: {}", err),
        };

        let nbytes: usize = io::stdin().read_line(&mut prompt)?;
        if nbytes == 0 {
            break;
        }
        run(prompt, had_err);
        *had_err = false;

        prompt = String::new();
    }
    Ok(())
}

fn run(source: String, had_err: &mut bool) {
    let mut scanner: Scanner = Scanner::new(&source);
    let tokens: &Vec<Token> = scanner.scan_tokens(had_err);
    let mut parser: Parser = Parser::new(tokens.clone(), had_err);
    let expression: Option<Expr> = parser.parse();

    if *had_err {
        return;
    }

    let myprinter: AstPrinter = AstPrinter {};
    match expression {
        Some(e) => println!("{}", myprinter.print(&e)),
        None => println!("nothing"),
    };
}
