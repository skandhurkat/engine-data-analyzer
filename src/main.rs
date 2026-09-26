use clap::{ArgAction, Parser};
use spdlog::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod parsers;
pub mod units;

use parsers::dynon::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg()]
    input_file_path: String,
    #[arg(short, long, default_value_t = 0, action = ArgAction::Count)]
    verbosity: u8,
}

fn main() {
    let args: Arguments = Arguments::parse();
    let file_path = &args.input_file_path;
    let log_level = if args.verbosity >= 5 {
        spdlog::Level::Trace
    } else if args.verbosity == 4 {
        spdlog::Level::Debug
    } else if args.verbosity == 3 {
        spdlog::Level::Info
    } else if args.verbosity == 2 {
        spdlog::Level::Warn
    } else if args.verbosity == 1 {
        spdlog::Level::Error
    } else {
        spdlog::Level::Critical
    };

    spdlog::default_logger().set_level_filter(spdlog::LevelFilter::MoreSevereEqual(log_level));

    println!("Reading {file_path}");

    let _ = try_parse_dynon(file_path);
}
