//use std::error::Error; // Importing the Error trait for error handling

use serde::Deserialize; // Importing the Deserialize trait from serde crate for JSON deserialization

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed to get response from website.")]
    RequestFailed(ureq::Error),
    #[error("Failed to turn response into string format.")]
    ToStringFailed(std::io::Error),
    #[error("Response failed to parse into JSON?")]
    FailedToSerdeJSON(serde_json::Error),

}

/// Struct representing the response containing multiple articles.
#[derive(Deserialize, Debug)] // Automatically generate code to deserialize and debug the struct
pub struct Articles {
    pub articles: Vec<Article>, // A vector containing multiple articles
}

/// Struct representing a single article.
#[derive(Deserialize, Debug)] // Automatically generate code to deserialize and debug the struct
pub struct Article {
    pub title: String, // The title of the article
    pub url: String,   // The URL of the article
}

/// Fetches articles from the given URL and returns a Result containing the Articles struct or an error.
pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    // Make a GET request to the given URL and get the response as a string
    let response = ureq::get(url).call().map_err(|e| NewsApiError::RequestFailed(e))
    ?.into_string().map_err(|e| NewsApiError::ToStringFailed(e))?;
    // Deserialize the JSON response into the Articles struct
    let articles: Articles = serde_json::from_str(&response).map_err(|e| NewsApiError::FailedToSerdeJSON(e))?;
    
    Ok(articles) // Return the articles wrapped in a Result
}