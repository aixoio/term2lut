use std::{env, process};

use term2lut::{lut::LUT, parser::ColorPalletVHS};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("usage: [source_file] [out_file]");
        process::exit(1);
    }

    let source_file_path = &args[1];
    let out_file_path = &args[2];

    println!("Loading {source_file_path}...");

    let color_pallet =
        ColorPalletVHS::load_file_to_color_pallet(source_file_path).unwrap_or_else(|err| {
            eprintln!("Paring error: {err}");
            process::exit(1);
        });

    let lut = LUT::new(33, color_pallet);
}
