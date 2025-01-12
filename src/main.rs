use clap::Parser;
use webbrowser;


#[derive(Parser)]
#[command(author = "Mike Oleshchuk", version = "0.1.0", about = "Oxi a tool to search the internet for 
        related programming and bug articles and websites.", long_about = None)]
struct Cli {
    query: String,

    #[arg(default_value = "general")]
    language: String,

    #[arg(default_value = "focus")]
    command: String,

}

fn main() {
    let args = Cli::parse();
    let url = format!("https://www.google.com/search?q={}+site:stackoverflow.com+OR+site:reddit.com", args.query);
    // Get input
    if webbrowser::open(&url).is_ok() {
        println!("Opened: {}", args.query);
    } else {
        println!("Failed to open query");
    }

}
