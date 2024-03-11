use base64::encode;
use clap::Parser;
use std::{fs, io::Write, io};
use anyhow::Result;

/// Prints basic info about this crate.
fn initial_print() {
    print!("{}, {}, built for ", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    #[cfg(target_os="linux")]
    println!("linux");
    #[cfg(target_os="windows")]
    println!("windows");
    #[cfg(target_os="macos")]
    println!("macos");
}

#[derive(Parser)]
#[command(version, about = "This sure feels like telling a toddler to fly a plane.")]
/// Both `input` & `output` must be specified.
struct Arguments {
    /// Input file (when specified).
    #[arg(short, long)]
    input: Option<String>,

    /// Output file (when specified).
    #[arg(short, long)]
    output: Option<String>
}

fn stdin_encode() -> Result<()> {
    println!("--input and --output parameters not (correctly) given. Insert text: ");
    let mut text = String::new();
    match io::stdin().read_line(&mut text) {
        Ok(_) => { println!("Encoded text: {}.", encode(text.trim().as_bytes())); },
        Err(_) => { println!("Error reading input. Aborted.");}
    }
    Ok(())
}

fn file_encode(files: Arguments) -> Result<()> {
    let text= fs::read_to_string(files.input.clone().unwrap());
    let mut encoded_text = String::new();
    match text {
        Ok(s) => { encoded_text = encode( s.as_bytes())},
        Err(_) => { println!("Error opening {}. Aborted.", files.input.clone().unwrap());}
    }
    let output_file = fs::File::create(files.output.clone().unwrap());
    match output_file {
        Ok(mut f) => { f.write_all(encoded_text.as_bytes())?; },
        Err(_) => { println!("Error creating {}. Aborted.", files.input.unwrap()); }
    }
    Ok(())
}

fn main() {
    initial_print();
    let args = Arguments::parse();
    if args.input.is_none() || args.output.is_none() {
        let _ = stdin_encode();
    }
    else {
        let _ = file_encode(args);
    }
}
