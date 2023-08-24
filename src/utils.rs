use rand::distributions::Alphanumeric;
use rand::Rng;
use scraper::{Html, Selector};
use std::error::Error;

pub fn generate(len: usize) -> String {
    let rng = rand::thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    random_string
}
pub async fn get_title_from_url(url: &str) -> Result<String, Box<dyn Error>> {
    // Send an HTTP GET request to the URL
    let response = reqwest::get(url).await?;

    // Read the response body as bytes
    let bytes = response.bytes().await?;

    // Convert the response body to a string
    let body = String::from_utf8(bytes.to_vec())?;

    // Parse the HTML using the scraper crate
    let fragment = Html::parse_document(&body);

    // Define a CSS selector to target the <title> tag
    let title_selector = Selector::parse("title").unwrap();

    // Extract the title text
    let title = fragment
        .select(&title_selector)
        .next()
        .map(|element| element.inner_html());

    // Return the extracted title (or a default value if none is found)
    Ok(title.unwrap_or_default())
}
