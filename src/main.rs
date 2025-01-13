use clap::{Parser, ValueEnum};


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

    // Search URL based on command type
    let url = match args.command {
        SearchType::Focus => format!(
            "https://www.google.com/search?q={}{}+site:medium.com.com+OR+site:reddit.com+OR+site:stackexchange.com+OR+site:stackoverflow.com",
            language_query, args.query
        ),
        SearchType::Broad => format!(
            "https://www.google.com/search?q={}{}",
            language_query, args.query
        ), 
    };

    // Attempt to open URL
    if webbrowser::open(&url).is_ok() {
        println!("Opened: {}", args.query); 
    } else {
        println!("Failed to open query");
    }

}
