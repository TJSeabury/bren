use clap::Parser;

/// A simple CLI utility for batch renaming.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The pattern
    #[clap(short, long, value_parser)]
    pattern: String,

    /// The prefix
    #[clap(short, long, value_parser)]
    prefix: String,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", to_uppercase(args.pattern));
    println!("{:?}", to_uppercase(args.prefix));
}

fn to_uppercase(word: String) -> String {
    let upper: String = word.to_uppercase();
    upper
}
