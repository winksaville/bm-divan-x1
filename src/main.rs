use std::env;
use std::process::ExitCode;

use bm_divan_x1::loops;
//use crate::loops; // does not work, because it's binary crate and library crate(?)

pub fn usage(args: Vec<String>) {
    let app_name = &args[0];
    println!("Usage {app_name} <number of loops>");
    println!("   loops: number of loops to run");
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        usage(args);
        return ExitCode::SUCCESS;
    }
    let loop_count: i64 = args[1].parse().expect("Unable to parse loop count to integer");
    let count = loops(loop_count);
    println!("Loop count: {count}");
    ExitCode::SUCCESS
}
