use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

extern crate argparse;

use argparse as ap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Arguments {
    input_file_path: String,
}

fn parse_args() -> Arguments {
    let mut file_path: String = "".to_string();
    {
        let mut ap = ap::ArgumentParser::new();
        ap.set_description("Parse and read an engine data monitor file");
        ap.refer(&mut file_path).required().add_argument(
            "file_name",
            ap::Store,
            r#"File to parse"#,
        );
        ap.parse_args_or_exit();
    }
    Arguments {
        input_file_path: file_path,
    }
}

fn main() {
    let args: Arguments = parse_args();
    let file_path = &args.input_file_path;
    println!("Reading {file_path}");

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            println!("{line}");
        }
    }
}
