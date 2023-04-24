use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

mod scanner;
mod token;
mod token_type;
mod errors;
mod parser;
mod expr;
mod interpreter;
use interpreter::Interpreter;
use scanner::Scanner;
use parser::Parser;
use errors::Log;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() > 2 {
        panic!("usage: rlox [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let file = File::open(path);
    if file.is_err() {
        println!("open {} error {:?}", path, file.unwrap());
        panic!();
    }

    let mut reader = BufReader::new(file.unwrap());
    let mut buffer: Vec<u8> = Vec::new();
    let ret = reader.read_to_end(&mut buffer);
    if ret.is_err() {
        println!("read file error:{:?}", ret.unwrap());
        panic!();
    }

    let s = String::from_utf8(buffer);
    if s.is_err() {
        println!("convert to string error {:?}", s.clone().err().unwrap());
        panic!();
    }

    let mut log = Log::new();
    run(&s.unwrap(), &mut log);

    if log.had_parse_error() {
        panic!("parse error, quit");
    }
    if log.had_runtime_error() {
        panic!("runtime error, quit");
    }
}

fn run_prompt() {
    let input = std::io::stdin();
    let mut reader = BufReader::new(input);

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        let ret = reader.read_line(&mut line);
        if ret.is_err() {
            println!("read stdin error {}", ret.unwrap());
            panic!();
        }

        let len = ret.unwrap();
        if len == 0 {
            break;
        }

        let mut log = Log::new();
        run(&line, &mut log);
        // had_error = false;
    }
}

fn run(s: &str, log: &mut Log) {
    let mut scanner = Scanner::new(s, log);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(&tokens, log);
    let ret = parser.parse();
    if let Ok(expr) = ret {
        let mut inter = Interpreter::new(log);
        inter.interpret(&expr);

    }
}
