use std::env;
use std::fs::{self, ReadDir};
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;

trait Exitable {
    fn report_and_exit(&self);
}

fn main() -> Result<(), ExecutionError> {
    let raw_args: Vec<String> = env::args().collect();
    let args = parse_args(raw_args);

    let res = match args {
        Ok(args) => {
            let exec_result = exec_tri(args);

            match exec_result {
                Ok(_) => { () }
                Err(exec_error) => {
                    exec_error.report_and_exit();
                }
            }
        }
        Err(parse_error) => {
            parse_error.report_and_exit();
        }
    };

    Ok(res)
}

#[derive(Debug)]
struct Args {
    path: ReadDir,
}

#[derive(Debug)]
enum ParseError {
    NoPathProvided,
    NotFound,
}

// TODO! should parse errors know how to exit?
impl Exitable for ParseError {
    fn report_and_exit(&self) {
        match self {
            ParseError::NoPathProvided => {
                std::process::exit(1);
            }
            ParseError::NotFound => {
                std::process::exit(1);
            }
            unk => {
                std::process::exit(1);
            }
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            ErrorKind::NotFound => {
                ParseError::NotFound
            }
            e => {
                eprintln!("io error ignored: {}", e);
                todo!()
            }
        }
    }
}

#[derive(Debug)]
enum ExecutionError {
    InvalidPath
}

impl Exitable for ExecutionError {
    fn report_and_exit(&self) {
        todo!()
    }
}

impl From<io::Error> for ExecutionError {
    fn from(value: Error) -> Self {
        todo!()
    }
}

fn parse_args(args: Vec<String>) -> Result<Args, ParseError> {
    let path = args.get(1)
        .ok_or(ParseError::NoPathProvided)?;

    let result = fs::read_dir(path);

    match result {
        Ok(path) => {
            Ok(Args {
                path,
            })
        },
        Err(_) => {
            Err(ParseError::NotFound)
        }
    }
}

fn exec_tri(args: Args) -> Result<(), ExecutionError> {
    let Args { path } = args;

    for entry in path {
        let path = entry?.path();
        let file_name = path.file_name().unwrap();
        let file_name = file_name.to_str().unwrap();
        println!("{}", file_name);
    }

    Ok(())
}


fn print_recursively(read_dir: &ReadDir) {
    todo!()
}