use clap::Parser;
use open;

#[derive(Parser)]
#[command(author = "Mike Oleshchuk", version = "0.1.0", about = "Demo", long_about = None)]
struct Cli {
    input: String,

}

fn main() {
    let args = Cli::parse();

    // Get input
    println!("Input: {}", args.input);

}
