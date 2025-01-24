# oxi

Oxi is a short script command-line interface (CLI) tool designed to streamline internet searches for programming-related questions, bugs, or articles. Built with Rust, it allows users to quickly query multiple developer-focused platforms directly from their terminal.

## Features
- Focus and Broad Search Modes: Search specifically within popular programming platforms or across the web.
- Multi-Word Query Support: Seamlessly handle multi-word queries with ease.
- Language Filtering: Optionally specify a programming language to refine search results.
- Customizable Sites: Search Stack Overflow, Reddit, Medium, and Stack Exchange.
- Quick URL Opening: Automatically opens the search in your default web browser.

## Installation
1. Ensure you have Rust and Cargo installed. If not, install them from [rustup.rs.](https://rustup.rs/).
 <br>
 
2. Clone this repository:
```bash
git clone https://github.com/Tawxyn/oxi.git
```
3. Navigate to the project directory:
```bash
cd oxi
```
4. Build the project:
```bash
cargo build --release
```
5. (Optional) Install the tool globally:
```bash
cargo install --path .
```

## Usage
```bash
oxi <QUERY> [LANGUAGE] [COMMAND]
```
### Arguments

- `<QUERY>`: The main search query (required).

- `[LANGUAGE]`: The programming language to filter results (optional; defaults to general).

- `[COMMAND]`: The type of search to perform (optional; defaults to focus).

    - focus: Searches within programming-related platforms.

    - broad: Performs a general web search.

    - video: Searches within youtube.

### Examples
- Seach for ownership issues:
```bash
oxi "ownership issues"
```
- Search for Rust syntax errors:
```bash
oxi "syntax error" rust
```
- Perform a broad search for Python memory leaks:
```bash
oxi "memory leak" python broad
```
