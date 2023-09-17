use std::env;
use std::{fs, io};

trait Exitable {
    fn report_and_exit(&self);
}

fn main() -> Result<(), ExecutionError> {
    let raw_args: Vec<String> = env::args().collect();
    let args = parse_args(raw_args);

    let res = match args {
        Ok(args) => {
            let exec_result = exec_tri(args, 0);

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
    path: String,
    dir_content: fs::ReadDir,
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
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::NotFound => {
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
}

impl Exitable for ExecutionError {
    fn report_and_exit(&self) {
        todo!()
    }
}

impl From<io::Error> for ExecutionError {
    fn from(value: io::Error) -> Self {
        todo!()
    }
}

fn parse_args(args: Vec<String>) -> Result<Args, ParseError> {
    let path = args.get(1)
        .ok_or(ParseError::NoPathProvided)?;

    let result = fs::read_dir(path);

    match result {
        Ok(dir_content) => {
            // TODO! kill the unwrap
            Ok(Args {
                path: path.split("/")
                    .last()
                    .unwrap()
                    .to_string(),
                dir_content,
            })
        }
        Err(_) => {
            Err(ParseError::NotFound)
        }
    }
}

fn exec_tri(args: Args, level: i32) -> Result<(), ExecutionError> {
    let Args { path, dir_content } = args;

    if level == 0  {
        println!("{}", path);
    }

    let mut peekable_dirs = dir_content.peekable();

    while let Some(entry) = peekable_dirs.next() {

        let path = &entry?.path();

        if path.is_dir() {
            let dir_content = fs::read_dir(path)?;
            let args = Args {
                path: path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                dir_content,
            };

            exec_tri(args, level + 1)?;
        }

        if peekable_dirs.peek().is_some() {
            println!("{}├── {}", get_identation(level), path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap());

            continue;
        }

        println!("{}└── {}", get_identation(level), path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap())
    }

    Ok(())
}

fn get_identation(level: i32) -> String {
    let mut identation = String::new();

    for _ in 0..level {
        identation.push_str("│   ");
    }

    identation
}