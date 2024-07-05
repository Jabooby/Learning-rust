/*!
This module fetches and displays articles from an API.

# Features of Rust Highlighted:
- **Error Handling:** Usage of `Result` and `Box<dyn Error>` for managing potential errors.
- **Serde for Deserialization:** The `serde` crate is used for converting JSON data into Rust structs.
- **Crate Dependencies:** Usage of external crates like `ureq` for making HTTP requests and `colour` for colored output.
- **Structs and Traits:** Definition of custom data types using `struct` and implementing traits like `Deserialize` and `Debug`.
*/
mod theme;

use std::error::Error; // Importing the Error trait for error handling

use colour::{dark_green, yellow}; // Importing functions for colored terminal output from the colour crate

use newsapi::{Articles, get_articles};

use dotenv::dotenv;


/// Renders the articles to the terminal with colored output.
fn render_articles(articles: &Articles) {
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    for a in &articles.articles {
        // Print the title in dark green
        //dark_green!("> {}\n", a.title);
        theme.print_text(&format!("`{}`", a.title));
        // Print the URL in yellow
        //yellow!("- {}\n\n", a.url);     
        theme.print_text(&format!(">*{}*", a.url));
        theme.print_text("---");
    }
}

/// The main function that serves as the entry point of the program.
fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;
    // The URL for fetching the top headlines from the news API
    let url: &str = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";

    let url = format!("{}{}", url, api_key);
    // Fetch the articles from the URL
    let articles = get_articles(&url)?;
    // Render the fetched articles to the terminal
    render_articles(&articles);
    
    Ok(()) // Return Ok indicating the program executed successfully
}
