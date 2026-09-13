extern crate argparse;

use argparse as ap;

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
}
