use std::{env, fs, process};

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
    let lut_string = lut.generate_lut_to_string();

    fs::write(out_file_path, lut_string).unwrap_or_else(|err| {
        eprintln!("Writing error: {err}");
        process::exit(1);
    });

    println!(
        "3D LUT generated {}x{}x{} and saved in {out_file_path}",
        lut.size, lut.size, lut.size
    );
}
