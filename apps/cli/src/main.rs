/*
    Appellation: acme-cli
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Initial efforts are being placed towards designing a project management suite of utilities
        for building scalable, user-centric dApps.
 */
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser, default_value = "World")]
    name: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}

// TODO - Create a well-designed input structure
struct Commands {
    file_path: String,
    port: u16,
    name: String,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}