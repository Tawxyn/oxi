use clap::{Parser, ValueEnum};
use webbrowser;


#[derive(Parser)]
#[command(author = "Mike Oleshchuk", version = "0.1.0", about = "Oxi a tool to search the internet for 
        related programming and bug articles and websites.", long_about = None)]
struct Cli {
    // Main query to search for
    query: String,

    // Language filter
    #[arg(default_value = "general", required = false)]
    language: String,

    // Search type
    #[arg(default_value = "focus", value_enum)]
    command: SearchType,

}

#[derive(ValueEnum, Clone)]
enum SearchType {
    Focus,
    Broad,
}

fn main() {
    // Initalize CLI tool with clap
    let args = Cli::parse();
    
    let language_query = if args.language == "general" {
        "".to_string()
    } else {
        format!("{} ", args.language)
    };
    // Format for search
    let url = format!("https://www.google.com/search?q={}{}+site:stackoverflow.com+OR+site:reddit.com+OR+site:stackexchange.com+OR+medium.com", language_query, args.query);

    // Attempt to open URL
    if webbrowser::open(&url).is_ok() {
        println!("Opened: {}", args.query); 
    } else {
        println!("Failed to open query");
    }

}
