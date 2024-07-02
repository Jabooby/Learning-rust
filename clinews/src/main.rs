/*!
This module fetches and displays articles from an API.

# Features of Rust Highlighted:
- **Error Handling:** Usage of `Result` and `Box<dyn Error>` for managing potential errors.
- **Serde for Deserialization:** The `serde` crate is used for converting JSON data into Rust structs.
- **Crate Dependencies:** Usage of external crates like `ureq` for making HTTP requests and `colour` for colored output.
- **Structs and Traits:** Definition of custom data types using `struct` and implementing traits like `Deserialize` and `Debug`.
*/

use std::error::Error; // Importing the Error trait for error handling

use serde::Deserialize; // Importing the Deserialize trait from serde crate for JSON deserialization

use colour::{dark_green, yellow}; // Importing functions for colored terminal output from the colour crate

/// Struct representing the response containing multiple articles.
#[derive(Deserialize, Debug)] // Automatically generate code to deserialize and debug the struct
struct Articles {
    articles: Vec<Article>, // A vector containing multiple articles
}

/// Struct representing a single article.
#[derive(Deserialize, Debug)] // Automatically generate code to deserialize and debug the struct
struct Article {
    title: String, // The title of the article
    url: String,   // The URL of the article
}

/// Fetches articles from the given URL and returns a Result containing the Articles struct or an error.
fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    // Make a GET request to the given URL and get the response as a string
    let response = ureq::get(url).call()?.into_string()?;
    // Deserialize the JSON response into the Articles struct
    let articles: Articles = serde_json::from_str(&response)?;
    
    Ok(articles) // Return the articles wrapped in a Result
}

/// Renders the articles to the terminal with colored output.
fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        // Print the title in dark green
        dark_green!("> {}\n", a.title);
        // Print the URL in yellow
        yellow!("- {}\n\n", a.url);
    }
}

/// The main function that serves as the entry point of the program.
fn main() -> Result<(), Box<dyn Error>> {
    // The URL for fetching the top headlines from the news API
    let url: &str = "https://newsapi.org/v2/top-headlines?country=us&apiKey=f2f3b4a62c7a404f8b7297b1f84ccfaf";
    // Fetch the articles from the URL
    let articles = get_articles(url)?;
    // Render the fetched articles to the terminal
    render_articles(&articles);
    
    Ok(()) // Return Ok indicating the program executed successfully
}
