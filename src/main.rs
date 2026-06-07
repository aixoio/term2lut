use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("usage: [source_file] [out_file]");
        process::exit(1);
    }
}
